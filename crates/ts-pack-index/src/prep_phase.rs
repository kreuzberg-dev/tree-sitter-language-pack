use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::pathing;
use crate::swift;
use crate::{
    FileImportEdgeRow, FileNode, ImplicitImportSymbolEdgeRow, ImportSymbolEdgeRow, ImportSymbolRequest,
    InferredCallRow, LaunchEdgeRow, PythonFileContext, PythonInferredCallRow, SwiftFileContext, SymbolNode,
};

pub(crate) struct PreparationOutputs {
    pub(crate) file_import_edges: Vec<FileImportEdgeRow>,
    pub(crate) import_symbol_edges: Vec<ImportSymbolEdgeRow>,
    pub(crate) implicit_import_symbol_edges: Vec<ImplicitImportSymbolEdgeRow>,
    pub(crate) inferred_call_rows: Vec<InferredCallRow>,
    pub(crate) python_inferred_call_rows: Vec<PythonInferredCallRow>,
    pub(crate) launch_edges: Vec<LaunchEdgeRow>,
}

pub(crate) fn prepare_graph_facts(
    all_symbols: &HashMap<&'static str, Vec<SymbolNode>>,
    all_files: &[FileNode],
    project_id: &Arc<str>,
    project_root: Option<&str>,
    launch_requests: &[(String, String)],
    import_symbol_requests: &[ImportSymbolRequest],
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
        let Some(target_fp) = pathing::resolve_file_import_target(
            &req.src_filepath,
            &req.module,
            &files_set,
            &swift_module_map,
            &stems,
        ) else {
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

    PreparationOutputs {
        file_import_edges,
        import_symbol_edges,
        implicit_import_symbol_edges,
        inferred_call_rows,
        python_inferred_call_rows,
        launch_edges,
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
            name: filepath.rsplit('/').next().unwrap_or(filepath).to_string(),
            filepath: filepath.to_string(),
            project_id: Arc::from("proj"),
        }
    }

    fn symbol_node(id: &str, name: &str, filepath: &str, is_exported: bool) -> SymbolNode {
        SymbolNode {
            id: id.to_string(),
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
            &[],
            &requests,
            &HashMap::new(),
            &[],
            &[],
        );

        assert_eq!(out.import_symbol_edges.len(), 1);
        assert_eq!(out.import_symbol_edges[0].src, "file:src/index.ts");
        assert_eq!(out.import_symbol_edges[0].tgt, "sym:server:buildRouter");
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
                &[],
                &[],
                &HashMap::new(),
                &[swift_ctx],
                &[],
            )
        });

        assert!(out.implicit_import_symbol_edges.iter().any(|edge| {
            edge.src == "file:main" && edge.tgt == "sym:public"
        }));
        assert!(!out.implicit_import_symbol_edges.iter().any(|edge| edge.tgt == "sym:internal"));

        let _ = std::fs::remove_dir_all(project_root);
    }
}
