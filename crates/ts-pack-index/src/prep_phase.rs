use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use tree_sitter_language_pack as ts_pack;

use crate::asset_phase;
use crate::call_resolution::{CallResolutionContext, build_symbol_call_rows, normalize_qualified_hint};
use crate::pathing;
use crate::swift;
use crate::{
    ApiRouteCallRow, ApiRouteHandlerRow, CallRef, CargoCrateFileRow, CargoCrateRow, CargoDependencyEdgeRow,
    CargoWorkspaceCrateRow, CargoWorkspaceRow, ExportAliasRequest, FileEdgeRow, FileImportEdgeRow, FileNode,
    GoFileContext, ImplicitImportSymbolEdgeRow, ImportSymbolEdgeRow, ImportSymbolRequest, InferredCallRow,
    LaunchEdgeRow, PythonFileContext, PythonInferredCallRow, ReExportSymbolRequest, ResourceBackingRow,
    ResourceTargetEdgeRow, ResourceUsageRow, RustFileContext, RustImplTraitEdgeRow, RustImplTypeEdgeRow,
    SwiftFileContext,
    SymbolCallRow, SymbolNode, XcodeSchemeFileRow, XcodeSchemeRow, XcodeSchemeTargetRow, XcodeTargetFileRow,
    XcodeTargetRow, XcodeWorkspaceProjectRow, XcodeWorkspaceRow,
};

pub(crate) struct PreparationOutputs {
    pub(crate) file_import_edges: Vec<FileImportEdgeRow>,
    pub(crate) asset_links: Vec<FileEdgeRow>,
    pub(crate) api_edges: Vec<FileEdgeRow>,
    pub(crate) api_route_calls: Vec<ApiRouteCallRow>,
    pub(crate) api_route_handlers: Vec<ApiRouteHandlerRow>,
    pub(crate) service_edges: Vec<FileEdgeRow>,
    pub(crate) resource_usages: Vec<ResourceUsageRow>,
    pub(crate) resource_backings: Vec<ResourceBackingRow>,
    pub(crate) xcode_targets: Vec<XcodeTargetRow>,
    pub(crate) xcode_target_files: Vec<XcodeTargetFileRow>,
    pub(crate) xcode_target_resources: Vec<ResourceTargetEdgeRow>,
    pub(crate) xcode_workspaces: Vec<XcodeWorkspaceRow>,
    pub(crate) xcode_workspace_projects: Vec<XcodeWorkspaceProjectRow>,
    pub(crate) xcode_schemes: Vec<XcodeSchemeRow>,
    pub(crate) xcode_scheme_targets: Vec<XcodeSchemeTargetRow>,
    pub(crate) xcode_scheme_files: Vec<XcodeSchemeFileRow>,
    pub(crate) cargo_crates: Vec<CargoCrateRow>,
    pub(crate) cargo_workspaces: Vec<CargoWorkspaceRow>,
    pub(crate) cargo_workspace_crates: Vec<CargoWorkspaceCrateRow>,
    pub(crate) cargo_crate_files: Vec<CargoCrateFileRow>,
    pub(crate) cargo_dependency_edges: Vec<CargoDependencyEdgeRow>,
    pub(crate) import_symbol_edges: Vec<ImportSymbolEdgeRow>,
    pub(crate) export_symbol_edges: Vec<crate::ExportSymbolEdgeRow>,
    pub(crate) export_alias_edges: Vec<crate::ExportAliasEdgeRow>,
    pub(crate) implicit_import_symbol_edges: Vec<ImplicitImportSymbolEdgeRow>,
    pub(crate) rust_impl_trait_edges: Vec<RustImplTraitEdgeRow>,
    pub(crate) rust_impl_type_edges: Vec<RustImplTypeEdgeRow>,
    pub(crate) symbol_call_rows: Vec<SymbolCallRow>,
    pub(crate) external_symbol_nodes: Vec<crate::ExternalSymbolNode>,
    pub(crate) external_symbol_edges: Vec<crate::ExternalSymbolEdgeRow>,
    pub(crate) inferred_call_rows: Vec<InferredCallRow>,
    pub(crate) python_inferred_call_rows: Vec<PythonInferredCallRow>,
    pub(crate) launch_edges: Vec<LaunchEdgeRow>,
}

pub(crate) fn prepare_graph_facts(
    all_symbols: &HashMap<&'static str, Vec<SymbolNode>>,
    all_files: &[FileNode],
    project_id: &Arc<str>,
    project_root: Option<&str>,
    manifest_abs: &HashMap<String, String>,
    file_facts: &HashMap<String, ts_pack::FileFacts>,
    call_refs: Vec<CallRef>,
    launch_requests: &[(String, String)],
    import_symbol_requests: &[ImportSymbolRequest],
    reexport_symbol_requests: &[ReExportSymbolRequest],
    export_alias_requests: &[ExportAliasRequest],
    swift_extension_map: &HashMap<String, HashSet<String>>,
    swift_contexts: &[SwiftFileContext],
    python_contexts: &[PythonFileContext],
    rust_contexts: &[RustFileContext],
    go_contexts: &[GoFileContext],
) -> PreparationOutputs {
    let debug_call_resolution = std::env::var("TS_PACK_DEBUG_CALL_RESOLUTION")
        .ok()
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    let mut symbols_by_file: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut caller_qualified_symbols_by_id: HashMap<String, String> = HashMap::new();
    let mut go_import_aliases_by_file: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut go_var_types_by_file: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut rust_var_types_by_file: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut go_method_return_types: HashMap<String, String> = HashMap::new();
    let mut go_function_return_types: HashMap<String, Vec<(String, String)>> = HashMap::new();
    let mut python_module_aliases_by_file: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut python_imported_symbol_modules_by_file: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut exported_symbols_by_file: HashMap<String, Vec<String>> = HashMap::new();
    let mut exported_symbols_by_prefix: HashMap<String, Vec<String>> = HashMap::new();
    let mut callable_symbols_by_name: HashMap<String, Vec<(String, String)>> = HashMap::new();
    let mut qualified_callable_symbols: Vec<(String, String, String)> = Vec::new();
    for syms in all_symbols.values() {
        for sym in syms {
            symbols_by_file
                .entry(sym.filepath.clone())
                .or_default()
                .insert(sym.name.clone(), sym.id.clone());
            if matches!(sym.kind.as_str(), "Function" | "Class" | "Struct" | "Method") {
                callable_symbols_by_name
                    .entry(sym.name.clone())
                    .or_default()
                    .push((sym.id.clone(), sym.filepath.clone()));
                if let Some(qualified_name) = sym.qualified_name.as_ref() {
                    caller_qualified_symbols_by_id.insert(sym.id.clone(), qualified_name.clone());
                    qualified_callable_symbols.push((
                        normalize_qualified_hint(qualified_name),
                        sym.id.clone(),
                        sym.filepath.clone(),
                    ));
                }
            }
            if sym.is_exported {
                exported_symbols_by_file
                    .entry(sym.filepath.clone())
                    .or_default()
                    .push(sym.id.clone());
                if let Some((prefix, _)) = sym.filepath.split_once('/') {
                    exported_symbols_by_prefix
                        .entry(prefix.to_string())
                        .or_default()
                        .push(sym.id.clone());
                }
            }
        }
    }
    for ctx in go_contexts {
        if !ctx.import_aliases.is_empty() {
            go_import_aliases_by_file.insert(ctx.filepath.clone(), ctx.import_aliases.clone());
        }
        for (method_key, return_type) in &ctx.method_return_types {
            go_method_return_types
                .entry(method_key.clone())
                .or_insert_with(|| return_type.clone());
        }
        for (function_name, return_type) in &ctx.function_return_types {
            go_function_return_types
                .entry(function_name.clone())
                .or_default()
                .push((ctx.filepath.clone(), return_type.clone()));
        }
        if !ctx.var_types.is_empty() {
            go_var_types_by_file.insert(ctx.filepath.clone(), ctx.var_types.clone());
        }
    }
    for ctx in go_contexts {
        if ctx.method_return_assignments.is_empty() {
            if ctx.function_return_assignments.is_empty() {
                continue;
            }
        }
        let file_var_types = go_var_types_by_file.entry(ctx.filepath.clone()).or_default();
        let mut changed = true;
        while changed {
            changed = false;
            for assignment in &ctx.method_return_assignments {
                if file_var_types.contains_key(&assignment.var_name) {
                    continue;
                }
                let Some(receiver_type) = file_var_types.get(&assignment.receiver_var) else {
                    continue;
                };
                let normalized_receiver = receiver_type
                    .split('.')
                    .next_back()
                    .unwrap_or(receiver_type)
                    .trim_start_matches('*')
                    .trim();
                if normalized_receiver.is_empty() {
                    continue;
                }
                let method_key = format!("{normalized_receiver}.{}", assignment.method_name);
                let Some(return_type) = go_method_return_types.get(&method_key) else {
                    continue;
                };
                file_var_types.insert(assignment.var_name.clone(), return_type.clone());
                changed = true;
            }
            for assignment in &ctx.function_return_assignments {
                if file_var_types.contains_key(&assignment.var_name) {
                    continue;
                }
                let Some(candidates) = go_function_return_types.get(&assignment.function_name) else {
                    continue;
                };
                let caller_dir = std::path::Path::new(&ctx.filepath)
                    .parent()
                    .and_then(|p| p.to_str())
                    .unwrap_or("");
                let mut local = candidates
                    .iter()
                    .filter(|(fp, _)| {
                        std::path::Path::new(fp).parent().and_then(|p| p.to_str()).unwrap_or("") == caller_dir
                    })
                    .map(|(_, ty)| ty.clone())
                    .collect::<Vec<_>>();
                local.sort();
                local.dedup();
                let inferred = if local.len() == 1 {
                    local.into_iter().next()
                } else {
                    let mut global = candidates.iter().map(|(_, ty)| ty.clone()).collect::<Vec<_>>();
                    global.sort();
                    global.dedup();
                    if global.len() == 1 {
                        global.into_iter().next()
                    } else {
                        None
                    }
                };
                let Some(return_type) = inferred else {
                    continue;
                };
                file_var_types.insert(assignment.var_name.clone(), return_type);
                changed = true;
            }
        }
    }

    let file_id_by_path: HashMap<String, String> =
        all_files.iter().map(|f| (f.filepath.clone(), f.id.clone())).collect();
    let files_set: HashSet<String> = all_files.iter().map(|f| f.filepath.clone()).collect();
    let mut imported_target_files_by_src: HashMap<String, HashSet<String>> = HashMap::new();
    let mut rust_local_module_roots_by_src_root: HashMap<String, HashSet<String>> = HashMap::new();
    let mut stems: HashMap<String, Vec<String>> = HashMap::new();
    for fp in &files_set {
        let stem = std::path::Path::new(fp)
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("")
            .to_string();
        if !stem.is_empty() {
            stems.entry(stem).or_default().push(fp.clone());
        }
        if fp.ends_with(".rs")
            && let Some(src_idx) = fp.find("/src/")
        {
            let src_root = fp[..src_idx + 4].to_string();
            let within_src = &fp[src_idx + 5..];
            let root = if let Some((first, _)) = within_src.split_once('/') {
                first.strip_suffix(".rs").unwrap_or(first)
            } else {
                within_src.strip_suffix(".rs").unwrap_or(within_src)
            };
            if !root.is_empty() && !matches!(root, "lib" | "main" | "mod") {
                rust_local_module_roots_by_src_root
                    .entry(src_root)
                    .or_default()
                    .insert(root.to_string());
            }
        }
    }

    for req in import_symbol_requests.iter() {
        if let Some(target_fp) = pathing::resolve_module_path(&req.src_filepath, &req.module, &files_set) {
            if target_fp != req.src_filepath {
                imported_target_files_by_src
                    .entry(req.src_filepath.clone())
                    .or_default()
                    .insert(target_fp);
            }
        }
    }
    for req in reexport_symbol_requests.iter() {
        if let Some(target_fp) = pathing::resolve_module_path(&req.src_filepath, &req.module, &files_set) {
            if target_fp != req.src_filepath {
                imported_target_files_by_src
                    .entry(req.src_filepath.clone())
                    .or_default()
                    .insert(target_fp);
            }
        }
    }
    for ctx in python_contexts {
        if !ctx.module_aliases.is_empty() {
            python_module_aliases_by_file.insert(ctx.filepath.clone(), ctx.module_aliases.clone());
        }
        if !ctx.imported_symbol_modules.is_empty() {
            python_imported_symbol_modules_by_file.insert(ctx.filepath.clone(), ctx.imported_symbol_modules.clone());
        }
    }
    for ctx in rust_contexts {
        if !ctx.var_types.is_empty() {
            rust_var_types_by_file.insert(ctx.filepath.clone(), ctx.var_types.clone());
        }
    }

    let resolve_import_item_for_file = |src_filepath: &str, item_name: &str| -> Option<String> {
        for req in import_symbol_requests
            .iter()
            .filter(|req| req.src_filepath == src_filepath)
        {
            let target_fp = pathing::resolve_module_path(&req.src_filepath, &req.module, &files_set);
            let sym_map = target_fp.as_ref().and_then(|fp| symbols_by_file.get(fp));
            if req.items.is_empty() {
                if let Some(fp) = target_fp.as_ref() {
                    if let Some(sym_map) = sym_map {
                        if let Some(sym_id) = sym_map.get(item_name) {
                            return Some(sym_id.clone());
                        }
                    } else if let Some(exported) = exported_symbols_by_file.get(fp) {
                        if let Some(sym_id) = exported.first() {
                            return Some(sym_id.clone());
                        }
                    }
                }
                continue;
            }
            if !req
                .items
                .iter()
                .any(|item| pathing::clean_import_name(item) == item_name)
            {
                continue;
            }
            if let Some(sym_map) = sym_map {
                if let Some(sym_id) = sym_map.get(item_name) {
                    return Some(sym_id.clone());
                }
            }
        }
        None
    };

    let resolution_ctx = CallResolutionContext {
        callable_symbols_by_name: &callable_symbols_by_name,
        qualified_callable_symbols: &qualified_callable_symbols,
        caller_qualified_symbols_by_id: &caller_qualified_symbols_by_id,
        symbols_by_file: &symbols_by_file,
        go_import_aliases_by_file: &go_import_aliases_by_file,
        go_var_types_by_file: &go_var_types_by_file,
        rust_var_types_by_file: &rust_var_types_by_file,
        python_module_aliases_by_file: &python_module_aliases_by_file,
        python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
        imported_target_files_by_src: &imported_target_files_by_src,
        import_symbol_requests,
        exported_symbols_by_file: &exported_symbols_by_file,
        files_set: &files_set,
        rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
    };

    let crate::call_resolution::CallResolutionOutputs {
        symbol_call_rows,
        external_symbol_nodes,
        external_symbol_edges,
        resolved_call_rows,
        unresolved_internal_call_rows,
        resolution_stage_counts,
        filtered_stage_counts,
        unresolved_name_counts,
        unresolved_bucket_counts,
        unresolved_bucket_samples,
        unresolved_rust_plain_attribution,
        skipped_external_call_rows,
    } = build_symbol_call_rows(call_refs, &resolution_ctx, project_id, debug_call_resolution);
    eprintln!(
        "[ts-pack-index] CALL resolve prep — resolved={} unresolved_internal={} filtered={}",
        resolved_call_rows, unresolved_internal_call_rows, skipped_external_call_rows,
    );
    if debug_call_resolution && !resolution_stage_counts.is_empty() {
        let mut stage_rows: Vec<(&'static str, usize)> = resolution_stage_counts.into_iter().collect();
        stage_rows.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));
        let summary = stage_rows
            .into_iter()
            .map(|(stage, count)| format!("{stage}:{count}"))
            .collect::<Vec<_>>()
            .join(" ");
        eprintln!("[ts-pack-index] CALL resolve stages — {summary}");
    }
    if debug_call_resolution && !filtered_stage_counts.is_empty() {
        let mut stage_rows: Vec<(&'static str, usize)> = filtered_stage_counts.into_iter().collect();
        stage_rows.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));
        let summary = stage_rows
            .into_iter()
            .map(|(stage, count)| format!("{stage}:{count}"))
            .collect::<Vec<_>>()
            .join(" ");
        eprintln!("[ts-pack-index] CALL filtered stages — {summary}");
    }
    if debug_call_resolution && !unresolved_name_counts.is_empty() {
        let mut top_unresolved: Vec<(String, usize)> = unresolved_name_counts.into_iter().collect();
        top_unresolved.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        top_unresolved.truncate(12);
        let summary = top_unresolved
            .into_iter()
            .map(|(name, count)| format!("{name}:{count}"))
            .collect::<Vec<_>>()
            .join(" ");
        eprintln!("[ts-pack-index] CALL unresolved top — {summary}");
    }
    if debug_call_resolution && !unresolved_bucket_counts.is_empty() {
        let mut bucket_rows: Vec<(String, usize)> = unresolved_bucket_counts.into_iter().collect();
        bucket_rows.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        bucket_rows.truncate(12);
        let summary = bucket_rows
            .into_iter()
            .map(|(bucket, count)| format!("{bucket}:{count}"))
            .collect::<Vec<_>>()
            .join(" ");
        eprintln!("[ts-pack-index] CALL unresolved buckets — {summary}");
    }
    if debug_call_resolution && !unresolved_bucket_samples.is_empty() {
        let mut sample_rows: Vec<(String, Vec<String>)> = unresolved_bucket_samples.into_iter().collect();
        sample_rows.sort_by(|a, b| a.0.cmp(&b.0));
        for (bucket, samples) in sample_rows.into_iter().filter(|(bucket, _)| {
            matches!(
                bucket.as_str(),
                "rust:plain:norecv"
                    | "rust:scoped:norecv"
                    | "go:scoped:recv"
                    | "python:scoped:recv"
                    | "python:member:recv"
                    | "python:plain:norecv"
            )
        }) {
            eprintln!(
                "[ts-pack-index] CALL unresolved samples [{bucket}] — {}",
                samples.join(" | ")
            );
        }
    }
    if debug_call_resolution && !unresolved_rust_plain_attribution.is_empty() {
        let mut rows: Vec<((String, String), usize)> = unresolved_rust_plain_attribution.into_iter().collect();
        rows.sort_by(|a, b| {
            b.1.cmp(&a.1)
                .then_with(|| a.0.0.cmp(&b.0.0))
                .then_with(|| a.0.1.cmp(&b.0.1))
        });
        rows.truncate(12);
        let summary = rows
            .into_iter()
            .map(|((callee, filepath), count)| format!("{callee}@{filepath}:{count}"))
            .collect::<Vec<_>>()
            .join(" | ");
        eprintln!("[ts-pack-index] CALL unresolved rust plain attribution — {summary}");
    }

    let mut launch_edges = Vec::new();
    if std::env::var("TS_PACK_LAUNCH_EDGES")
        .ok()
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
    {
        let project_root_str = project_root.unwrap_or("");
        let mut seen_launch: HashSet<(String, String)> = HashSet::new();
        for (src_fp, raw) in launch_requests {
            let Some(tgt_fp) = pathing::resolve_launch_path(src_fp, raw, project_root_str, &files_set) else {
                continue;
            };
            if src_fp == &tgt_fp {
                continue;
            }
            if seen_launch.insert((src_fp.clone(), tgt_fp.clone())) {
                launch_edges.push(LaunchEdgeRow {
                    src_filepath: src_fp.clone(),
                    tgt_filepath: tgt_fp,
                    project_id: project_id.to_string(),
                });
            }
        }
    }

    let swift_module_map = project_root
        .map(|root| pathing::build_swift_module_map(root, &files_set))
        .unwrap_or_default();
    let mut swift_file_modules: HashMap<String, Vec<String>> = HashMap::new();
    for (module, module_files) in &swift_module_map {
        for fp in module_files {
            swift_file_modules.entry(fp.clone()).or_default().push(module.clone());
        }
    }

    let mut file_import_edges = Vec::new();
    let mut seen_file_import_edges: HashSet<(String, String)> = HashSet::new();
    for req in import_symbol_requests.iter() {
        let Some(src_id) = file_id_by_path.get(&req.src_filepath) else {
            continue;
        };
        let Some(target_fp) =
            pathing::resolve_file_import_target(&req.src_filepath, &req.module, &files_set, &swift_module_map, &stems)
        else {
            continue;
        };
        if target_fp == req.src_filepath {
            continue;
        }
        let Some(_target_id) = file_id_by_path.get(&target_fp) else {
            continue;
        };
        if seen_file_import_edges.insert((src_id.clone(), target_fp.clone())) {
            file_import_edges.push(FileImportEdgeRow {
                src_filepath: req.src_filepath.clone(),
                tgt_filepath: target_fp,
                project_id: project_id.to_string(),
            });
        }
    }

    let mut import_symbol_edges = Vec::new();
    let mut seen_import_symbol: HashSet<(String, String)> = HashSet::new();
    for req in import_symbol_requests.iter() {
        let target_fp = pathing::resolve_module_path(&req.src_filepath, &req.module, &files_set);
        let sym_map = target_fp.as_ref().and_then(|fp| symbols_by_file.get(fp));
        if req.items.is_empty() {
            if let Some(fp) = target_fp.as_ref() {
                if let Some(exported) = exported_symbols_by_file.get(fp) {
                    for sym_id in exported {
                        if seen_import_symbol.insert((req.src_id.clone(), sym_id.clone())) {
                            import_symbol_edges.push(ImportSymbolEdgeRow {
                                src: req.src_id.clone(),
                                tgt: sym_id.clone(),
                            });
                        }
                    }
                    continue;
                }
            }
            if req.src_filepath.ends_with(".swift") {
                if let Some(module_files) = swift_module_map.get(&req.module) {
                    for fp in module_files {
                        if let Some(exported) = exported_symbols_by_file.get(fp) {
                            for sym_id in exported {
                                if seen_import_symbol.insert((req.src_id.clone(), sym_id.clone())) {
                                    import_symbol_edges.push(ImportSymbolEdgeRow {
                                        src: req.src_id.clone(),
                                        tgt: sym_id.clone(),
                                    });
                                }
                            }
                        }
                    }
                    continue;
                }
            }
            if let Some(prefix) = req.module.split('.').next().filter(|p| !p.is_empty()) {
                if let Some(exported) = exported_symbols_by_prefix.get(prefix) {
                    for sym_id in exported {
                        if seen_import_symbol.insert((req.src_id.clone(), sym_id.clone())) {
                            import_symbol_edges.push(ImportSymbolEdgeRow {
                                src: req.src_id.clone(),
                                tgt: sym_id.clone(),
                            });
                        }
                    }
                }
            }
            continue;
        }

        for item in &req.items {
            let name = pathing::clean_import_name(item);
            if name.is_empty() {
                continue;
            }
            if let Some(sym_map) = sym_map {
                if let Some(sym_id) = sym_map.get(&name) {
                    if seen_import_symbol.insert((req.src_id.clone(), sym_id.clone())) {
                        import_symbol_edges.push(ImportSymbolEdgeRow {
                            src: req.src_id.clone(),
                            tgt: sym_id.clone(),
                        });
                    }
                }
            }
        }
    }

    let mut export_symbol_edges = Vec::new();
    let mut seen_export_symbol: HashSet<(String, String)> = HashSet::new();
    for req in reexport_symbol_requests.iter() {
        let target_fp = pathing::resolve_module_path(&req.src_filepath, &req.module, &files_set);
        let sym_map = target_fp.as_ref().and_then(|fp| symbols_by_file.get(fp));

        if req.is_wildcard || req.items.is_empty() {
            if let Some(fp) = target_fp.as_ref() {
                if let Some(exported) = exported_symbols_by_file.get(fp) {
                    for sym_id in exported {
                        if seen_export_symbol.insert((req.src_id.clone(), sym_id.clone())) {
                            export_symbol_edges.push(crate::ExportSymbolEdgeRow {
                                src: req.src_id.clone(),
                                tgt: sym_id.clone(),
                            });
                        }
                    }
                }
            }
            continue;
        }

        for item in &req.items {
            let name = pathing::clean_import_name(item);
            if name.is_empty() {
                continue;
            }
            if let Some(sym_map) = sym_map {
                if let Some(sym_id) = sym_map.get(&name) {
                    if seen_export_symbol.insert((req.src_id.clone(), sym_id.clone())) {
                        export_symbol_edges.push(crate::ExportSymbolEdgeRow {
                            src: req.src_id.clone(),
                            tgt: sym_id.clone(),
                        });
                    }
                }
            }
        }
    }

    let mut export_alias_edges = Vec::new();
    let mut seen_export_alias: HashSet<(String, String, String)> = HashSet::new();
    for req in export_alias_requests.iter() {
        if req.item == "*" {
            let Some(module) = req.module.as_ref().filter(|module| !module.is_empty()) else {
                continue;
            };
            let target_fp = pathing::resolve_module_path(&req.src_filepath, module, &files_set);
            let Some(fp) = target_fp.as_ref() else {
                continue;
            };
            let Some(exported) = exported_symbols_by_file.get(fp) else {
                continue;
            };
            for sym_id in exported {
                if seen_export_symbol.insert((req.src_id.clone(), sym_id.clone())) {
                    export_symbol_edges.push(crate::ExportSymbolEdgeRow {
                        src: req.src_id.clone(),
                        tgt: sym_id.clone(),
                    });
                }
                if seen_export_alias.insert((req.src_id.clone(), sym_id.clone(), req.exported_as.clone())) {
                    export_alias_edges.push(crate::ExportAliasEdgeRow {
                        src: req.src_id.clone(),
                        tgt: sym_id.clone(),
                        exported_as: req.exported_as.clone(),
                    });
                }
            }
            continue;
        }

        let target_sym = if let Some(module) = req.module.as_ref().filter(|module| !module.is_empty()) {
            let target_fp = pathing::resolve_module_path(&req.src_filepath, module, &files_set);
            let sym_map = target_fp.as_ref().and_then(|fp| symbols_by_file.get(fp));
            sym_map.and_then(|map| map.get(&req.item).cloned())
        } else {
            symbols_by_file
                .get(&req.src_filepath)
                .and_then(|map| map.get(&req.item).cloned())
                .or_else(|| resolve_import_item_for_file(&req.src_filepath, &req.item))
        };
        if let Some(sym_id) = target_sym {
            if seen_export_symbol.insert((req.src_id.clone(), sym_id.clone())) {
                export_symbol_edges.push(crate::ExportSymbolEdgeRow {
                    src: req.src_id.clone(),
                    tgt: sym_id.clone(),
                });
            }
            if seen_export_alias.insert((req.src_id.clone(), sym_id.clone(), req.exported_as.clone())) {
                export_alias_edges.push(crate::ExportAliasEdgeRow {
                    src: req.src_id.clone(),
                    tgt: sym_id,
                    exported_as: req.exported_as.clone(),
                });
            }
        }
    }

    let mut implicit_import_symbol_edges = Vec::new();
    let swift_implicit_imports = std::env::var("TS_PACK_ENABLE_HEURISTIC_IMPLICIT_IMPORTS_SYMBOL")
        .ok()
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    if swift_implicit_imports {
        let mut seen_implicit_import_symbol: HashSet<(String, String)> = HashSet::new();
        for (src_fp, modules) in &swift_file_modules {
            let Some(src_id) = file_id_by_path.get(src_fp) else {
                continue;
            };
            for module in modules {
                if let Some(module_files) = swift_module_map.get(module) {
                    for fp in module_files {
                        if fp == src_fp {
                            continue;
                        }
                        if let Some(exported) = exported_symbols_by_file.get(fp) {
                            for sym_id in exported {
                                if seen_import_symbol.contains(&(src_id.clone(), sym_id.clone())) {
                                    continue;
                                }
                                if seen_implicit_import_symbol.insert((src_id.clone(), sym_id.clone())) {
                                    implicit_import_symbol_edges.push(ImplicitImportSymbolEdgeRow {
                                        src: src_id.clone(),
                                        tgt: sym_id.clone(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let allow_same_file = std::env::var("TS_PACK_INCLUDE_INTRA_FILE_CALLS")
        .ok()
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    let mut inferred_call_rows = Vec::new();
    if !swift_extension_map.is_empty() && !swift_contexts.is_empty() {
        let mut seen: HashSet<(String, String, String)> = HashSet::new();
        for ctx in swift_contexts {
            for call in &ctx.call_sites {
                let Some(recv_raw) = &call.receiver else {
                    continue;
                };
                let recv = recv_raw.trim_end_matches(|c| c == '?' || c == '!');
                if recv.is_empty() {
                    continue;
                }

                let mut norm_ty = ctx.var_types.get(recv).and_then(|t| swift::normalize_swift_type(t));
                if norm_ty.is_none() {
                    if recv == "self" || recv == "Self" {
                        norm_ty = ctx
                            .extension_spans
                            .iter()
                            .filter(|(sb, eb, _)| *sb <= call.start_byte && call.start_byte < *eb)
                            .min_by_key(|(sb, eb, _)| eb - sb)
                            .map(|(_, _, ty)| ty.clone())
                            .or_else(|| {
                                ctx.type_spans
                                    .iter()
                                    .filter(|(sb, eb, _)| *sb <= call.start_byte && call.start_byte < *eb)
                                    .min_by_key(|(sb, eb, _)| eb - sb)
                                    .map(|(_, _, ty)| ty.clone())
                            });
                    } else if swift_extension_map.contains_key(recv) {
                        norm_ty = swift::normalize_swift_type(recv);
                    }
                }

                let Some(norm_ty) = norm_ty else {
                    continue;
                };

                let mut candidates = Vec::new();
                if let Some(methods) = swift_extension_map.get(&norm_ty) {
                    if methods.contains(&call.callee) {
                        candidates.push(norm_ty.clone());
                    }
                }
                if candidates.is_empty() {
                    if let Some((_, short)) = norm_ty.rsplit_once('.') {
                        if let Some(methods) = swift_extension_map.get(short) {
                            if methods.contains(&call.callee) {
                                candidates.push(short.to_string());
                            }
                        }
                    }
                }
                if candidates.is_empty() {
                    continue;
                }

                let caller_id = ctx
                    .symbol_spans
                    .iter()
                    .filter(|(sb, eb, _)| *sb <= call.start_byte && call.start_byte < *eb)
                    .min_by_key(|(sb, eb, _)| eb - sb)
                    .map(|(_, _, id)| id.clone())
                    .unwrap_or_else(|| ctx.file_id.clone());

                for ty in candidates {
                    if seen.insert((caller_id.clone(), call.callee.clone(), ty.clone())) {
                        inferred_call_rows.push(InferredCallRow {
                            caller_id: caller_id.clone(),
                            callee: call.callee.clone(),
                            receiver_type: ty,
                            project_id: Arc::clone(project_id),
                            caller_filepath: ctx.filepath.clone(),
                            allow_same_file,
                        });
                    }
                }
            }
        }
    }

    let mut python_inferred_call_rows = Vec::new();
    let python_attr_calls = std::env::var("TS_PACK_PY_ATTR_CALLS")
        .ok()
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    if python_attr_calls && !python_contexts.is_empty() {
        let mut seen: HashSet<(String, String, String)> = HashSet::new();
        for ctx in python_contexts {
            for call in &ctx.call_sites {
                let Some(recv) = &call.receiver else {
                    continue;
                };
                let Some(module) = ctx.module_aliases.get(recv) else {
                    continue;
                };
                let Some(module_fp) = pathing::resolve_module_path(&ctx.filepath, module, &files_set) else {
                    continue;
                };
                let exact_resolves = symbols_by_file
                    .get(&module_fp)
                    .and_then(|sym_map| sym_map.get(&call.callee))
                    .is_some();
                if exact_resolves {
                    continue;
                }

                let caller_id = ctx
                    .symbol_spans
                    .iter()
                    .filter(|(sb, eb, _)| *sb <= call.start_byte && call.start_byte < *eb)
                    .min_by_key(|(sb, eb, _)| eb - sb)
                    .map(|(_, _, id)| id.clone())
                    .unwrap_or_else(|| ctx.file_id.clone());
                if seen.insert((caller_id.clone(), call.callee.clone(), module_fp.clone())) {
                    python_inferred_call_rows.push(PythonInferredCallRow {
                        caller_id,
                        callee: call.callee.clone(),
                        callee_filepath: module_fp,
                        project_id: Arc::clone(project_id),
                        caller_filepath: ctx.filepath.clone(),
                        allow_same_file,
                    });
                }
            }
        }
    }

    if !go_contexts.is_empty() {
        let mut seen: HashSet<(String, String, String)> = HashSet::new();
        for ctx in go_contexts {
            for call in &ctx.call_sites {
                let Some(recv) = &call.receiver else {
                    continue;
                };
                let Some(receiver_type) = go_var_types_by_file.get(&ctx.filepath).and_then(|m| m.get(recv)) else {
                    continue;
                };
                let normalized_type = receiver_type
                    .split('.')
                    .next_back()
                    .unwrap_or(receiver_type)
                    .trim_start_matches('*')
                    .trim();
                if normalized_type.is_empty() {
                    continue;
                }
                let caller_id = ctx
                    .symbol_spans
                    .iter()
                    .filter(|(sb, eb, _)| *sb <= call.start_byte && call.start_byte < *eb)
                    .min_by_key(|(sb, eb, _)| eb - sb)
                    .map(|(_, _, id)| id.clone())
                    .unwrap_or_else(|| ctx.file_id.clone());
                if seen.insert((caller_id.clone(), call.callee.clone(), normalized_type.to_string())) {
                    inferred_call_rows.push(InferredCallRow {
                        caller_id,
                        callee: call.callee.clone(),
                        receiver_type: normalized_type.to_string(),
                        project_id: Arc::clone(project_id),
                        caller_filepath: ctx.filepath.clone(),
                        allow_same_file,
                    });
                }
            }
        }
    }

    let mut rust_impl_trait_edges = Vec::new();
    let mut rust_impl_type_edges = Vec::new();
    let trait_symbols: HashMap<String, String> = all_symbols
        .get("Trait")
        .into_iter()
        .flat_map(|symbols| symbols.iter())
        .map(|sym| (sym.name.clone(), sym.id.clone()))
        .collect();
    let type_symbols: HashMap<String, String> = all_symbols
        .values()
        .flat_map(|symbols| symbols.iter())
        .filter(|sym| matches!(sym.kind.as_str(), "Struct" | "Class" | "Enum" | "TypeAlias"))
        .map(|sym| (sym.name.clone(), sym.id.clone()))
        .collect();
    let mut seen_impl_trait = HashSet::new();
    let mut seen_impl_type = HashSet::new();
    if let Some(impl_symbols) = all_symbols.get("Impl") {
        for sym in impl_symbols {
            let name = sym.name.as_str();
            if name.is_empty() {
                continue;
            }
            let (trait_name, type_name) = parse_rust_impl_targets(name);
            if let Some(trait_name) = trait_name
                && trait_symbols.contains_key(&trait_name)
                && seen_impl_trait.insert((sym.id.clone(), trait_name.clone()))
            {
                rust_impl_trait_edges.push(RustImplTraitEdgeRow {
                    impl_id: sym.id.clone(),
                    trait_name,
                    project_id: project_id.to_string(),
                });
            }
            if let Some(type_name) = type_name
                && type_symbols.contains_key(&type_name)
                && seen_impl_type.insert((sym.id.clone(), type_name.clone()))
            {
                rust_impl_type_edges.push(RustImplTypeEdgeRow {
                    impl_id: sym.id.clone(),
                    type_name,
                    project_id: project_id.to_string(),
                });
            }
        }
    }

    let asset = asset_phase::prepare_asset_graph_facts(all_files, file_facts, manifest_abs, project_id);

    PreparationOutputs {
        file_import_edges,
        asset_links: asset.asset_links,
        api_edges: asset.api_edges,
        api_route_calls: asset.api_route_calls,
        api_route_handlers: asset.api_route_handlers,
        service_edges: asset.service_edges,
        resource_usages: asset.resource_usages,
        resource_backings: asset.resource_backings,
        xcode_targets: asset.xcode_targets,
        xcode_target_files: asset.xcode_target_files,
        xcode_target_resources: asset.xcode_target_resources,
        xcode_workspaces: asset.xcode_workspaces,
        xcode_workspace_projects: asset.xcode_workspace_projects,
        xcode_schemes: asset.xcode_schemes,
        xcode_scheme_targets: asset.xcode_scheme_targets,
        xcode_scheme_files: asset.xcode_scheme_files,
        cargo_crates: asset.cargo_crates,
        cargo_workspaces: asset.cargo_workspaces,
        cargo_workspace_crates: asset.cargo_workspace_crates,
        cargo_crate_files: asset.cargo_crate_files,
        cargo_dependency_edges: asset.cargo_dependency_edges,
        import_symbol_edges,
        export_symbol_edges,
        export_alias_edges,
        implicit_import_symbol_edges,
        rust_impl_trait_edges,
        rust_impl_type_edges,
        symbol_call_rows,
        external_symbol_nodes,
        external_symbol_edges,
        inferred_call_rows,
        python_inferred_call_rows,
        launch_edges,
    }
}

fn parse_rust_impl_targets(name: &str) -> (Option<String>, Option<String>) {
    let Some(body) = name.trim().strip_prefix("impl ") else {
        return (None, None);
    };
    if let Some((trait_part, type_part)) = body.split_once(" for ") {
        (
            normalize_rust_impl_target_name(trait_part),
            normalize_rust_impl_target_name(type_part),
        )
    } else {
        (None, normalize_rust_impl_target_name(body))
    }
}

fn normalize_rust_impl_target_name(raw: &str) -> Option<String> {
    let trimmed = raw
        .split('{')
        .next()
        .unwrap_or(raw)
        .split(" where ")
        .next()
        .unwrap_or(raw)
        .trim()
        .trim_start_matches("dyn ")
        .trim_start_matches('&')
        .trim_start_matches("mut ")
        .trim();
    let base = trimmed.split('<').next().unwrap_or(trimmed).trim();
    let simple = base.split("::").last().unwrap_or(base).trim();
    if simple.is_empty() {
        None
    } else {
        Some(simple.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::tags::CallSite;

    fn file_node(id: &str, filepath: &str) -> FileNode {
        FileNode {
            id: id.to_string(),
            stable_id: id.to_string(),
            name: filepath.rsplit('/').next().unwrap_or(filepath).to_string(),
            filepath: filepath.to_string(),
            project_id: Arc::from("proj"),
            is_test: false,
        }
    }

    fn symbol_node(id: &str, name: &str, filepath: &str, is_exported: bool) -> SymbolNode {
        SymbolNode {
            id: id.to_string(),
            stable_id: id.to_string(),
            name: name.to_string(),
            kind: "Function".to_string(),
            qualified_name: Some(format!("{name}.qualified")),
            filepath: filepath.to_string(),
            project_id: Arc::from("proj"),
            start_line: 1,
            end_line: 2,
            start_byte: 0,
            end_byte: 50,
            signature: None,
            visibility: None,
            is_exported,
            doc_comment: None,
        }
    }

    fn with_env_var<R>(key: &str, value: Option<&str>, f: impl FnOnce() -> R) -> R {
        let prev = std::env::var(key).ok();
        unsafe {
            match value {
                Some(v) => std::env::set_var(key, v),
                None => std::env::remove_var(key),
            }
        }
        let result = f();
        unsafe {
            match prev.as_deref() {
                Some(v) => std::env::set_var(key, v),
                None => std::env::remove_var(key),
            }
        }
        result
    }

    fn unique_temp_dir(name: &str) -> PathBuf {
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        std::env::temp_dir().join(format!("ts-pack-index-prep-{name}-{nanos}"))
    }

    #[test]
    fn prepares_import_symbol_edges_for_named_imports() {
        let mut all_symbols = HashMap::new();
        all_symbols.insert(
            "Function",
            vec![symbol_node(
                "sym:server:buildRouter",
                "buildRouter",
                "src/api/routes.ts",
                true,
            )],
        );
        let all_files = vec![
            file_node("file:src/index.ts", "src/index.ts"),
            file_node("file:src/api/routes.ts", "src/api/routes.ts"),
        ];
        let requests = vec![ImportSymbolRequest {
            src_id: "file:src/index.ts".to_string(),
            src_filepath: "src/index.ts".to_string(),
            module: "./api/routes".to_string(),
            items: vec!["buildRouter".to_string()],
        }];

        let out = prepare_graph_facts(
            &all_symbols,
            &all_files,
            &Arc::from("proj"),
            None,
            &HashMap::new(),
            &HashMap::new(),
            vec![],
            &[],
            &requests,
            &[],
            &[],
            &HashMap::new(),
            &[],
            &[],
            &[],
            &[],
        );

        assert_eq!(out.import_symbol_edges.len(), 1);
        assert_eq!(out.import_symbol_edges[0].src, "file:src/index.ts");
        assert_eq!(out.import_symbol_edges[0].tgt, "sym:server:buildRouter");
    }

    #[test]
    fn prepares_export_symbol_edges_for_named_reexports() {
        let mut all_symbols = HashMap::new();
        all_symbols.insert(
            "TypeAlias",
            vec![
                symbol_node("sym:foo", "Foo", "src/types.ts", true),
                symbol_node("sym:bar", "Bar", "src/types.ts", true),
            ],
        );
        let all_files = vec![
            file_node("file:src/index.ts", "src/index.ts"),
            file_node("file:src/types.ts", "src/types.ts"),
        ];
        let requests = vec![ReExportSymbolRequest {
            src_id: "file:src/index.ts".to_string(),
            src_filepath: "src/index.ts".to_string(),
            module: "./types".to_string(),
            items: vec!["Foo".to_string(), "Bar".to_string()],
            is_wildcard: false,
        }];

        let out = prepare_graph_facts(
            &all_symbols,
            &all_files,
            &Arc::from("proj"),
            None,
            &HashMap::new(),
            &HashMap::new(),
            vec![],
            &[],
            &[],
            &requests,
            &[],
            &HashMap::new(),
            &[],
            &[],
            &[],
            &[],
        );

        assert_eq!(out.export_symbol_edges.len(), 2);
        assert!(
            out.export_symbol_edges
                .iter()
                .any(|edge| edge.src == "file:src/index.ts" && edge.tgt == "sym:foo")
        );
        assert!(
            out.export_symbol_edges
                .iter()
                .any(|edge| edge.src == "file:src/index.ts" && edge.tgt == "sym:bar")
        );
    }

    #[test]
    fn prepares_export_symbol_edges_for_wildcard_reexports() {
        let mut all_symbols = HashMap::new();
        all_symbols.insert(
            "Function",
            vec![
                symbol_node("sym:buildRouter", "buildRouter", "src/routes.ts", true),
                symbol_node("sym:privateThing", "privateThing", "src/routes.ts", false),
            ],
        );
        let all_files = vec![
            file_node("file:src/index.ts", "src/index.ts"),
            file_node("file:src/routes.ts", "src/routes.ts"),
        ];
        let requests = vec![ReExportSymbolRequest {
            src_id: "file:src/index.ts".to_string(),
            src_filepath: "src/index.ts".to_string(),
            module: "./routes".to_string(),
            items: Vec::new(),
            is_wildcard: true,
        }];

        let out = prepare_graph_facts(
            &all_symbols,
            &all_files,
            &Arc::from("proj"),
            None,
            &HashMap::new(),
            &HashMap::new(),
            vec![],
            &[],
            &[],
            &requests,
            &[],
            &HashMap::new(),
            &[],
            &[],
            &[],
            &[],
        );

        assert_eq!(out.export_symbol_edges.len(), 1);
        assert_eq!(out.export_symbol_edges[0].src, "file:src/index.ts");
        assert_eq!(out.export_symbol_edges[0].tgt, "sym:buildRouter");
    }

    #[test]
    fn prepares_export_alias_edges_for_local_alias_exports() {
        let mut all_symbols = HashMap::new();
        all_symbols.insert(
            "TypeAlias",
            vec![symbol_node("sym:routeContext", "RouteContext", "src/context.ts", true)],
        );
        let all_files = vec![file_node("file:src/context.ts", "src/context.ts")];
        let alias_requests = vec![ExportAliasRequest {
            src_id: "file:src/context.ts".to_string(),
            src_filepath: "src/context.ts".to_string(),
            module: None,
            item: "RouteContext".to_string(),
            exported_as: "PublicRouteContext".to_string(),
        }];

        let out = prepare_graph_facts(
            &all_symbols,
            &all_files,
            &Arc::from("proj"),
            None,
            &HashMap::new(),
            &HashMap::new(),
            vec![],
            &[],
            &[],
            &[],
            &alias_requests,
            &HashMap::new(),
            &[],
            &[],
            &[],
            &[],
        );

        assert_eq!(out.export_alias_edges.len(), 1);
        assert_eq!(out.export_alias_edges[0].src, "file:src/context.ts");
        assert_eq!(out.export_alias_edges[0].tgt, "sym:routeContext");
        assert_eq!(out.export_alias_edges[0].exported_as, "PublicRouteContext");
    }

    #[test]
    fn prepares_export_alias_edges_for_imported_local_alias_exports() {
        let mut all_symbols = HashMap::new();
        all_symbols.insert(
            "TypeAlias",
            vec![symbol_node("sym:config", "Config", "src/types.ts", true)],
        );
        let all_files = vec![
            file_node("file:src/client.ts", "src/client.ts"),
            file_node("file:src/types.ts", "src/types.ts"),
        ];
        let import_requests = vec![ImportSymbolRequest {
            src_id: "file:src/client.ts".to_string(),
            src_filepath: "src/client.ts".to_string(),
            module: "./types".to_string(),
            items: vec!["Config".to_string()],
        }];
        let alias_requests = vec![ExportAliasRequest {
            src_id: "file:src/client.ts".to_string(),
            src_filepath: "src/client.ts".to_string(),
            module: None,
            item: "Config".to_string(),
            exported_as: "PublicConfig".to_string(),
        }];

        let out = prepare_graph_facts(
            &all_symbols,
            &all_files,
            &Arc::from("proj"),
            None,
            &HashMap::new(),
            &HashMap::new(),
            vec![],
            &[],
            &import_requests,
            &[],
            &alias_requests,
            &HashMap::new(),
            &[],
            &[],
            &[],
            &[],
        );

        assert_eq!(out.export_alias_edges.len(), 1);
        assert_eq!(out.export_alias_edges[0].src, "file:src/client.ts");
        assert_eq!(out.export_alias_edges[0].tgt, "sym:config");
        assert_eq!(out.export_alias_edges[0].exported_as, "PublicConfig");
    }

    #[test]
    fn prepares_export_alias_edges_for_reexport_aliases() {
        let mut all_symbols = HashMap::new();
        all_symbols.insert("TypeAlias", vec![symbol_node("sym:foo", "Foo", "src/types.ts", true)]);
        let all_files = vec![
            file_node("file:src/index.ts", "src/index.ts"),
            file_node("file:src/types.ts", "src/types.ts"),
        ];
        let alias_requests = vec![ExportAliasRequest {
            src_id: "file:src/index.ts".to_string(),
            src_filepath: "src/index.ts".to_string(),
            module: Some("./types".to_string()),
            item: "Foo".to_string(),
            exported_as: "PublicFoo".to_string(),
        }];

        let out = prepare_graph_facts(
            &all_symbols,
            &all_files,
            &Arc::from("proj"),
            None,
            &HashMap::new(),
            &HashMap::new(),
            vec![],
            &[],
            &[],
            &[],
            &alias_requests,
            &HashMap::new(),
            &[],
            &[],
            &[],
            &[],
        );

        assert_eq!(out.export_alias_edges.len(), 1);
        assert_eq!(out.export_alias_edges[0].src, "file:src/index.ts");
        assert_eq!(out.export_alias_edges[0].tgt, "sym:foo");
        assert_eq!(out.export_alias_edges[0].exported_as, "PublicFoo");
    }

    #[test]
    fn prepares_namespace_export_alias_edges_for_wildcard_reexports() {
        let mut all_symbols = HashMap::new();
        all_symbols.insert(
            "Function",
            vec![symbol_node("sym:buildRouter", "buildRouter", "src/routes.ts", true)],
        );
        all_symbols.insert(
            "TypeAlias",
            vec![symbol_node("sym:routeConfig", "RouteConfig", "src/routes.ts", true)],
        );
        let all_files = vec![
            file_node("file:src/index.ts", "src/index.ts"),
            file_node("file:src/routes.ts", "src/routes.ts"),
        ];
        let alias_requests = vec![ExportAliasRequest {
            src_id: "file:src/index.ts".to_string(),
            src_filepath: "src/index.ts".to_string(),
            module: Some("./routes".to_string()),
            item: "*".to_string(),
            exported_as: "routes.*".to_string(),
        }];

        let out = prepare_graph_facts(
            &all_symbols,
            &all_files,
            &Arc::from("proj"),
            None,
            &HashMap::new(),
            &HashMap::new(),
            vec![],
            &[],
            &[],
            &[],
            &alias_requests,
            &HashMap::new(),
            &[],
            &[],
            &[],
            &[],
        );

        assert_eq!(out.export_alias_edges.len(), 2);
        assert_eq!(out.export_symbol_edges.len(), 2);
        assert!(out.export_alias_edges.iter().all(|row| row.exported_as == "routes.*"));
    }

    #[test]
    fn prepares_swift_inferred_calls_from_extension_context() {
        let mut swift_extension_map = HashMap::new();
        swift_extension_map.insert("Service".to_string(), HashSet::from(["run".to_string()]));
        let ctx = SwiftFileContext {
            file_id: "file:service.swift".to_string(),
            filepath: "Sources/App/service.swift".to_string(),
            symbol_spans: vec![(0, 100, "sym:caller".to_string())],
            extension_spans: vec![(0, 100, "Service".to_string())],
            type_spans: vec![],
            call_sites: vec![CallSite {
                start_byte: 10,
                callee: "run".to_string(),
                qualified_callee: None,
                receiver: Some("self".to_string()),
            }],
            var_types: HashMap::new(),
        };

        let out = prepare_graph_facts(
            &HashMap::new(),
            &[file_node("file:service.swift", "Sources/App/service.swift")],
            &Arc::from("proj"),
            None,
            &HashMap::new(),
            &HashMap::new(),
            vec![],
            &[],
            &[],
            &[],
            &[],
            &swift_extension_map,
            &[ctx],
            &[],
            &[],
            &[],
        );

        assert_eq!(out.inferred_call_rows.len(), 1);
        assert_eq!(out.inferred_call_rows[0].caller_id, "sym:caller");
        assert_eq!(out.inferred_call_rows[0].callee, "run");
        assert_eq!(out.inferred_call_rows[0].receiver_type, "Service");
    }

    #[test]
    fn prepares_go_inferred_calls_from_receiver_constructor_types() {
        let ctx = GoFileContext {
            file_id: "file:smoke_test.go".to_string(),
            filepath: "tests/test_apps/go/smoke_test.go".to_string(),
            symbol_spans: vec![(0, 200, "sym:go_test".to_string())],
            call_sites: vec![CallSite {
                start_byte: 25,
                callee: "Close".to_string(),
                qualified_callee: Some("registry.Close".to_string()),
                receiver: Some("registry".to_string()),
            }],
            import_aliases: HashMap::new(),
            var_types: HashMap::from([("registry".to_string(), "Registry".to_string())]),
            method_return_assignments: vec![],
            method_return_types: HashMap::new(),
            function_return_assignments: vec![],
            function_return_types: HashMap::new(),
        };

        let out = prepare_graph_facts(
            &HashMap::new(),
            &[file_node("file:smoke_test.go", "tests/test_apps/go/smoke_test.go")],
            &Arc::from("proj"),
            None,
            &HashMap::new(),
            &HashMap::new(),
            vec![],
            &[],
            &[],
            &[],
            &[],
            &HashMap::new(),
            &[],
            &[],
            &[],
            &[ctx],
        );

        assert_eq!(out.inferred_call_rows.len(), 1);
        assert_eq!(out.inferred_call_rows[0].caller_id, "sym:go_test");
        assert_eq!(out.inferred_call_rows[0].callee, "Close");
        assert_eq!(out.inferred_call_rows[0].receiver_type, "Registry");
    }

    #[test]
    fn prepares_go_inferred_calls_from_method_return_types() {
        let provider_ctx = GoFileContext {
            file_id: "file:packages/go/v1/tspack.go".to_string(),
            filepath: "packages/go/v1/tspack.go".to_string(),
            symbol_spans: vec![],
            call_sites: vec![],
            import_aliases: HashMap::new(),
            var_types: HashMap::new(),
            method_return_assignments: vec![],
            method_return_types: HashMap::from([("Registry.ParseString".to_string(), "Tree".to_string())]),
            function_return_assignments: vec![],
            function_return_types: HashMap::new(),
        };
        let caller_ctx = GoFileContext {
            file_id: "file:smoke_test.go".to_string(),
            filepath: "tests/test_apps/go/smoke_test.go".to_string(),
            symbol_spans: vec![(0, 300, "sym:go_test".to_string())],
            call_sites: vec![CallSite {
                start_byte: 125,
                callee: "RootNodeType".to_string(),
                qualified_callee: Some("tree.RootNodeType".to_string()),
                receiver: Some("tree".to_string()),
            }],
            import_aliases: HashMap::new(),
            var_types: HashMap::from([("registry".to_string(), "Registry".to_string())]),
            method_return_assignments: vec![crate::GoMethodReturnAssignment {
                var_name: "tree".to_string(),
                receiver_var: "registry".to_string(),
                method_name: "ParseString".to_string(),
            }],
            method_return_types: HashMap::new(),
            function_return_assignments: vec![],
            function_return_types: HashMap::new(),
        };

        let out = prepare_graph_facts(
            &HashMap::new(),
            &[
                file_node("file:smoke_test.go", "tests/test_apps/go/smoke_test.go"),
                file_node("file:packages/go/v1/tspack.go", "packages/go/v1/tspack.go"),
            ],
            &Arc::from("proj"),
            None,
            &HashMap::new(),
            &HashMap::new(),
            vec![],
            &[],
            &[],
            &[],
            &[],
            &HashMap::new(),
            &[],
            &[],
            &[],
            &[provider_ctx, caller_ctx],
        );

        assert_eq!(out.inferred_call_rows.len(), 1);
        assert_eq!(out.inferred_call_rows[0].caller_id, "sym:go_test");
        assert_eq!(out.inferred_call_rows[0].callee, "RootNodeType");
        assert_eq!(out.inferred_call_rows[0].receiver_type, "Tree");
    }

    #[test]
    fn prepares_go_inferred_calls_from_helper_function_return_types() {
        let helper_ctx = GoFileContext {
            file_id: "file:helpers_test.go".to_string(),
            filepath: "e2e/go/helpers_test.go".to_string(),
            symbol_spans: vec![],
            call_sites: vec![],
            import_aliases: HashMap::new(),
            var_types: HashMap::new(),
            method_return_assignments: vec![],
            method_return_types: HashMap::new(),
            function_return_assignments: vec![],
            function_return_types: HashMap::from([("newTestRegistry".to_string(), "Registry".to_string())]),
        };
        let caller_ctx = GoFileContext {
            file_id: "file:process_test.go".to_string(),
            filepath: "e2e/go/process_test.go".to_string(),
            symbol_spans: vec![(0, 300, "sym:test".to_string())],
            call_sites: vec![CallSite {
                start_byte: 120,
                callee: "Process".to_string(),
                qualified_callee: Some("reg.Process".to_string()),
                receiver: Some("reg".to_string()),
            }],
            import_aliases: HashMap::new(),
            var_types: HashMap::new(),
            method_return_assignments: vec![],
            method_return_types: HashMap::new(),
            function_return_assignments: vec![crate::GoFunctionReturnAssignment {
                var_name: "reg".to_string(),
                function_name: "newTestRegistry".to_string(),
            }],
            function_return_types: HashMap::new(),
        };

        let out = prepare_graph_facts(
            &HashMap::new(),
            &[
                file_node("file:helpers_test.go", "e2e/go/helpers_test.go"),
                file_node("file:process_test.go", "e2e/go/process_test.go"),
            ],
            &Arc::from("proj"),
            None,
            &HashMap::new(),
            &HashMap::new(),
            vec![],
            &[],
            &[],
            &[],
            &[],
            &HashMap::new(),
            &[],
            &[],
            &[],
            &[helper_ctx, caller_ctx],
        );
        assert_eq!(out.inferred_call_rows.len(), 1);
        assert_eq!(out.inferred_call_rows[0].callee, "Process");
        assert_eq!(out.inferred_call_rows[0].receiver_type, "Registry");
    }

    #[test]
    fn prepares_python_attribute_inferred_calls_when_enabled() {
        let all_files = vec![
            file_node("file:pkg/main.py", "pkg/main.py"),
            file_node("file:pkg/helpers.py", "pkg/helpers.py"),
        ];
        let ctx = PythonFileContext {
            file_id: "file:pkg/main.py".to_string(),
            filepath: "pkg/main.py".to_string(),
            symbol_spans: vec![(0, 100, "sym:main".to_string())],
            call_sites: vec![CallSite {
                start_byte: 5,
                callee: "run".to_string(),
                qualified_callee: None,
                receiver: Some("helpers".to_string()),
            }],
            module_aliases: HashMap::from([("helpers".to_string(), ".helpers".to_string())]),
            imported_symbol_modules: HashMap::new(),
        };

        let out = with_env_var("TS_PACK_PY_ATTR_CALLS", Some("1"), || {
            prepare_graph_facts(
                &HashMap::new(),
                &all_files,
                &Arc::from("proj"),
                None,
                &HashMap::new(),
                &HashMap::new(),
                vec![],
                &[],
                &[],
                &[],
                &[],
                &HashMap::new(),
                &[],
                &[ctx],
                &[],
                &[],
            )
        });

        assert_eq!(out.python_inferred_call_rows.len(), 1);
        assert_eq!(out.python_inferred_call_rows[0].caller_id, "sym:main");
        assert_eq!(out.python_inferred_call_rows[0].callee, "run");
        assert_eq!(out.python_inferred_call_rows[0].callee_filepath, "pkg/helpers.py");
    }

    #[test]
    fn python_exact_module_alias_calls_do_not_emit_inferred_rows() {
        let all_files = vec![
            file_node("file:pkg/main.py", "pkg/main.py"),
            file_node("file:pkg/helpers.py", "pkg/helpers.py"),
        ];
        let mut all_symbols = HashMap::new();
        all_symbols.insert("Function", vec![symbol_node("sym:run", "run", "pkg/helpers.py", true)]);
        let ctx = PythonFileContext {
            file_id: "file:pkg/main.py".to_string(),
            filepath: "pkg/main.py".to_string(),
            symbol_spans: vec![(0, 100, "sym:main".to_string())],
            call_sites: vec![CallSite {
                start_byte: 5,
                callee: "run".to_string(),
                qualified_callee: None,
                receiver: Some("helpers".to_string()),
            }],
            module_aliases: HashMap::from([("helpers".to_string(), ".helpers".to_string())]),
            imported_symbol_modules: HashMap::new(),
        };

        let out = with_env_var("TS_PACK_PY_ATTR_CALLS", Some("1"), || {
            prepare_graph_facts(
                &all_symbols,
                &all_files,
                &Arc::from("proj"),
                None,
                &HashMap::new(),
                &HashMap::new(),
                vec![],
                &[],
                &[],
                &[],
                &[],
                &HashMap::new(),
                &[],
                &[ctx],
                &[],
                &[],
            )
        });

        assert!(out.python_inferred_call_rows.is_empty());
    }

    #[test]
    fn prepares_swift_implicit_imports_for_exported_symbols_only() {
        let mut all_symbols = HashMap::new();
        all_symbols.insert(
            "Function",
            vec![
                symbol_node("sym:public", "PublicThing", "Sources/App/Public.swift", true),
                symbol_node("sym:internal", "InternalThing", "Sources/App/Internal.swift", false),
            ],
        );
        let all_files = vec![
            file_node("file:main", "Sources/App/Main.swift"),
            file_node("file:public", "Sources/App/Public.swift"),
            file_node("file:internal", "Sources/App/Internal.swift"),
        ];
        let swift_ctx = SwiftFileContext {
            file_id: "file:main".to_string(),
            filepath: "Sources/App/Main.swift".to_string(),
            symbol_spans: vec![],
            extension_spans: vec![],
            type_spans: vec![],
            call_sites: vec![],
            var_types: HashMap::new(),
        };
        let project_root = unique_temp_dir("swift-implicit-exports");
        std::fs::create_dir_all(project_root.join("Sources/App")).unwrap();
        std::fs::write(
            project_root.join("Package.swift"),
            r#"
            let package = Package(
              name: "Demo",
              targets: [
                .target(name: "App", path: "Sources/App")
              ]
            )
            "#,
        )
        .unwrap();

        let out = with_env_var("TS_PACK_ENABLE_HEURISTIC_IMPLICIT_IMPORTS_SYMBOL", Some("1"), || {
            prepare_graph_facts(
                &all_symbols,
                &all_files,
                &Arc::from("proj"),
                project_root.to_str(),
                &HashMap::new(),
                &HashMap::new(),
                vec![],
                &[],
                &[],
                &[],
                &[],
                &HashMap::new(),
                &[swift_ctx],
                &[],
                &[],
                &[],
            )
        });

        assert!(
            out.implicit_import_symbol_edges
                .iter()
                .any(|edge| { edge.src == "file:main" && edge.tgt == "sym:public" })
        );
        assert!(
            !out.implicit_import_symbol_edges
                .iter()
                .any(|edge| edge.tgt == "sym:internal")
        );

        let _ = std::fs::remove_dir_all(project_root);
    }

    #[test]
    fn prepares_rust_impl_trait_and_type_edges() {
        let mut all_symbols = HashMap::new();
        all_symbols.insert(
            "Trait",
            vec![SymbolNode {
                kind: "Trait".to_string(),
                ..symbol_node("sym:trait", "Runner", "src/lib.rs", true)
            }],
        );
        all_symbols.insert(
            "Struct",
            vec![SymbolNode {
                kind: "Struct".to_string(),
                ..symbol_node("sym:struct", "Service", "src/lib.rs", true)
            }],
        );
        all_symbols.insert(
            "Impl",
            vec![SymbolNode {
                kind: "Impl".to_string(),
                qualified_name: None,
                ..symbol_node("sym:impl", "impl Runner for Service", "src/lib.rs", false)
            }],
        );

        let out = prepare_graph_facts(
            &all_symbols,
            &[file_node("file:lib", "src/lib.rs")],
            &Arc::from("proj"),
            None,
            &HashMap::new(),
            &HashMap::new(),
            vec![],
            &[],
            &[],
            &[],
            &[],
            &HashMap::new(),
            &[],
            &[],
            &[],
            &[],
        );

        assert!(
            out.rust_impl_trait_edges
                .iter()
                .any(|edge| edge.impl_id == "sym:impl" && edge.trait_name == "Runner")
        );
        assert!(
            out.rust_impl_type_edges
                .iter()
                .any(|edge| edge.impl_id == "sym:impl" && edge.type_name == "Service")
        );
    }
}
