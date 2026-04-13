use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use tree_sitter_language_pack as ts_pack;

use crate::asset_phase;
use crate::pathing;
use crate::swift;
use crate::{
    ApiRouteCallRow, ApiRouteHandlerRow, CargoCrateFileRow, CargoCrateRow, CargoDependencyEdgeRow,
    CargoWorkspaceCrateRow, CargoWorkspaceRow, ExportAliasRequest, FileEdgeRow, FileImportEdgeRow, FileNode,
    ImplicitImportSymbolEdgeRow, ImportSymbolEdgeRow, ImportSymbolRequest, InferredCallRow, LaunchEdgeRow,
    PythonFileContext, PythonInferredCallRow, ReExportSymbolRequest, ResourceBackingRow, ResourceTargetEdgeRow,
    ResourceUsageRow, RustImplTraitEdgeRow, RustImplTypeEdgeRow, SwiftFileContext, SymbolNode, XcodeSchemeFileRow,
    XcodeSchemeRow, XcodeSchemeTargetRow, XcodeTargetFileRow, XcodeTargetRow, XcodeWorkspaceProjectRow,
    XcodeWorkspaceRow,
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
    launch_requests: &[(String, String)],
    import_symbol_requests: &[ImportSymbolRequest],
    reexport_symbol_requests: &[ReExportSymbolRequest],
    export_alias_requests: &[ExportAliasRequest],
    swift_extension_map: &HashMap<String, HashSet<String>>,
    swift_contexts: &[SwiftFileContext],
    python_contexts: &[PythonFileContext],
) -> PreparationOutputs {
    let mut symbols_by_file: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut exported_symbols_by_file: HashMap<String, Vec<String>> = HashMap::new();
    let mut exported_symbols_by_prefix: HashMap<String, Vec<String>> = HashMap::new();
    for syms in all_symbols.values() {
        for sym in syms {
            symbols_by_file
                .entry(sym.filepath.clone())
                .or_default()
                .insert(sym.name.clone(), sym.id.clone());
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

    let file_id_by_path: HashMap<String, String> =
        all_files.iter().map(|f| (f.filepath.clone(), f.id.clone())).collect();
    let files_set: HashSet<String> = all_files.iter().map(|f| f.filepath.clone()).collect();
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
    }

    let resolve_symbol_from_import_request = |src_filepath: &str, item_name: &str| -> Option<String> {
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
                        for sym_id in exported {
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
                .or_else(|| resolve_symbol_from_import_request(&req.src_filepath, &req.item))
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
            &[],
            &requests,
            &[],
            &[],
            &HashMap::new(),
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
            &[],
            &[],
            &requests,
            &[],
            &HashMap::new(),
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
            &[],
            &[],
            &requests,
            &[],
            &HashMap::new(),
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
            &[],
            &[],
            &[],
            &alias_requests,
            &HashMap::new(),
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
            &[],
            &import_requests,
            &[],
            &alias_requests,
            &HashMap::new(),
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
            &[],
            &[],
            &[],
            &alias_requests,
            &HashMap::new(),
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
            &[],
            &[],
            &[],
            &alias_requests,
            &HashMap::new(),
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
            &[],
            &[],
            &[],
            &[],
            &swift_extension_map,
            &[ctx],
            &[],
        );

        assert_eq!(out.inferred_call_rows.len(), 1);
        assert_eq!(out.inferred_call_rows[0].caller_id, "sym:caller");
        assert_eq!(out.inferred_call_rows[0].callee, "run");
        assert_eq!(out.inferred_call_rows[0].receiver_type, "Service");
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
                receiver: Some("helpers".to_string()),
            }],
            module_aliases: HashMap::from([("helpers".to_string(), ".helpers".to_string())]),
        };

        let out = with_env_var("TS_PACK_PY_ATTR_CALLS", Some("1"), || {
            prepare_graph_facts(
                &HashMap::new(),
                &all_files,
                &Arc::from("proj"),
                None,
                &HashMap::new(),
                &HashMap::new(),
                &[],
                &[],
                &[],
                &[],
                &HashMap::new(),
                &[],
                &[ctx],
            )
        });

        assert_eq!(out.python_inferred_call_rows.len(), 1);
        assert_eq!(out.python_inferred_call_rows[0].caller_id, "sym:main");
        assert_eq!(out.python_inferred_call_rows[0].callee, "run");
        assert_eq!(out.python_inferred_call_rows[0].callee_filepath, "pkg/helpers.py");
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
                &[],
                &[],
                &[],
                &[],
                &HashMap::new(),
                &[swift_ctx],
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
            &[],
            &[],
            &[],
            &[],
            &HashMap::new(),
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
