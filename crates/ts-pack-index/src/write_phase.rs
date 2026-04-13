use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, Instant};

use futures::{TryStreamExt, stream};
use neo4rs::Graph;

use crate::clone_enrich;
use crate::writers;
use crate::{
    ApiRouteCallRow, ApiRouteHandlerRow, CALLS_BATCH_SIZE, CALL_EDGE_BATCH_SIZE, CargoCrateFileRow, CargoCrateRow,
    CargoDependencyEdgeRow, CargoWorkspaceCrateRow, CargoWorkspaceRow, CloneCandidate, DbEdgeRow, DbModelEdgeRow,
    ExportAliasEdgeRow, ExportSymbolEdgeRow, ExternalApiEdgeRow, ExternalApiNode, FileEdgeRow, FileImportEdgeRow,
    FileNode, IMPORT_BATCH_SIZE, ImplicitImportSymbolEdgeRow, ImportNode, ImportSymbolEdgeRow, InferredCallRow,
    LaunchEdgeRow, NODE_BATCH_SIZE, NODE_CONCURRENCY, PythonInferredCallRow, REL_BATCH_SIZE, REL_CONCURRENCY, RelRow,
    ResourceBackingRow, ResourceTargetEdgeRow, ResourceUsageRow, RustImplTraitEdgeRow, RustImplTypeEdgeRow,
    SymbolCallRow, SymbolNode, XcodeSchemeFileRow, XcodeSchemeRow, XcodeSchemeTargetRow, XcodeTargetFileRow,
    XcodeTargetRow, XcodeWorkspaceProjectRow, XcodeWorkspaceRow, external_api_id, extract_prisma_models,
};

pub(crate) struct WritePhaseSummary {
    pub(crate) node_elapsed: Duration,
    pub(crate) import_elapsed: Duration,
    pub(crate) rel_elapsed: Duration,
    pub(crate) calls_elapsed: Duration,
}

pub(crate) struct WriteInputs {
    pub(crate) run_id: String,
    pub(crate) all_files: Vec<FileNode>,
    pub(crate) all_symbols: HashMap<&'static str, Vec<SymbolNode>>,
    pub(crate) all_imports: Vec<ImportNode>,
    pub(crate) all_rels: Vec<RelRow>,
    pub(crate) all_import_rels: Vec<RelRow>,
    pub(crate) all_symbol_call_rows: Vec<SymbolCallRow>,
    pub(crate) clone_candidates: Vec<CloneCandidate>,
    pub(crate) inferred_call_rows: Vec<InferredCallRow>,
    pub(crate) python_inferred_call_rows: Vec<PythonInferredCallRow>,
    pub(crate) db_sources: Vec<String>,
    pub(crate) db_model_refs_by_file: Vec<(String, String)>,
    pub(crate) external_api_edges: Vec<ExternalApiEdgeRow>,
    pub(crate) external_api_urls: HashSet<String>,
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
    pub(crate) implicit_import_symbol_edges: Vec<ImplicitImportSymbolEdgeRow>,
    pub(crate) rust_impl_trait_edges: Vec<RustImplTraitEdgeRow>,
    pub(crate) rust_impl_type_edges: Vec<RustImplTypeEdgeRow>,
    pub(crate) export_symbol_edges: Vec<ExportSymbolEdgeRow>,
    pub(crate) export_alias_edges: Vec<ExportAliasEdgeRow>,
    pub(crate) launch_edges: Vec<LaunchEdgeRow>,
    pub(crate) manifest_abs: HashMap<String, String>,
}

fn ok_chunks<'a, T>(
    items: &'a [T],
    chunk_size: usize,
) -> impl futures::stream::Stream<Item = Result<&'a [T], neo4rs::Error>> + 'a {
    stream::iter(items.chunks(chunk_size).map(Ok::<_, neo4rs::Error>))
}

pub(crate) async fn run_write_phases(
    graph: &Arc<Graph>,
    project_id: &Arc<str>,
    inputs: WriteInputs,
) -> neo4rs::Result<WritePhaseSummary> {
    // Symbol-edge writes touch the same File/Node relationship groups heavily and
    // have proven prone to Neo4j deadlocks when batched concurrently.
    let symbol_edge_concurrency = 1usize;
    // CALLS/CALLS_INFERRED writes also contend on the same caller/callee node
    // relationship groups. Serializing these batches avoids transient deadlocks
    // from overlapping MERGE lock acquisition across concurrent transactions.
    let call_edge_concurrency = 1usize;
    // CONTAINS writes touch shared parent/child node relationship groups. Even
    // with stable sort order, concurrent MERGE batches can still overlap on the
    // same node sets and trigger transient deadlocks. This phase is cheap enough
    // that serial execution is the safer default.
    let contains_rel_concurrency = 1usize;
    // CALLS_DB_MODEL writes MERGE shared File/Model pairs and can deadlock on
    // larger repos when chunked concurrently. Keep this path serialized too.
    let db_model_edge_concurrency = 1usize;

    let WriteInputs {
        run_id,
        all_files,
        mut all_symbols,
        all_imports,
        mut all_rels,
        all_import_rels,
        all_symbol_call_rows,
        clone_candidates,
        inferred_call_rows,
        python_inferred_call_rows,
        db_sources,
        db_model_refs_by_file,
        external_api_edges,
        external_api_urls,
        file_import_edges,
        asset_links,
        api_edges,
        api_route_calls,
        api_route_handlers,
        service_edges,
        resource_usages,
        resource_backings,
        xcode_targets,
        xcode_target_files,
        xcode_target_resources,
        xcode_workspaces,
        xcode_workspace_projects,
        xcode_schemes,
        xcode_scheme_targets,
        xcode_scheme_files,
        cargo_crates,
        cargo_workspaces,
        cargo_workspace_crates,
        cargo_crate_files,
        cargo_dependency_edges,
        import_symbol_edges,
        implicit_import_symbol_edges,
        rust_impl_trait_edges,
        rust_impl_type_edges,
        export_symbol_edges,
        export_alias_edges,
        launch_edges,
        manifest_abs,
    } = inputs;

    let schema_id = all_files
        .iter()
        .find(|f| f.filepath == "prisma/schema.prisma")
        .map(|f| f.id.clone());
    if let Some(schema_id) = schema_id {
        let mut seen: HashSet<String> = HashSet::new();
        let mut db_edges = Vec::new();
        for src in &db_sources {
            if seen.insert(src.clone()) {
                db_edges.push(DbEdgeRow {
                    src: src.clone(),
                    tgt: schema_id.clone(),
                });
            }
        }
        if !db_edges.is_empty() {
            db_edges.sort_by(|a, b| a.src.cmp(&b.src).then_with(|| a.tgt.cmp(&b.tgt)));
            let t_db = Instant::now();
            let db_count = db_edges.len();
            ok_chunks(&db_edges, CALLS_BATCH_SIZE)
                .try_for_each_concurrent(REL_CONCURRENCY, |chunk| {
                    let g = Arc::clone(graph);
                    let run_id = run_id.clone();
                    async move { writers::write_db_edges(&g, chunk, &run_id).await }
                })
                .await?;
            eprintln!(
                "[ts-pack-index] CALLS_DB writes done in {:.2}s (rows={})",
                t_db.elapsed().as_secs_f64(),
                db_count,
            );
        }
    }

    let mut model_map: HashMap<String, String> = HashMap::new();
    if let Some(schema_abs) = manifest_abs.get("prisma/schema.prisma")
        && let Ok(schema_text) = std::fs::read_to_string(schema_abs)
    {
        for model in extract_prisma_models(&schema_text) {
            if model.is_empty() {
                continue;
            }
            model_map.insert(model.to_lowercase(), model.clone());
            if let Some(first) = model.chars().next() {
                let delegate = first.to_lowercase().collect::<String>() + &model[1..];
                model_map.insert(delegate.to_lowercase(), model.clone());
            }
        }
    }

    let mut db_model_edges = Vec::new();
    for (file_id, model_ref) in &db_model_refs_by_file {
        let model = model_map
            .get(&model_ref.to_lowercase())
            .cloned()
            .unwrap_or_else(|| model_ref.clone());
        db_model_edges.push(DbModelEdgeRow {
            src: file_id.clone(),
            model,
            project_id: Arc::clone(project_id),
        });
    }
    if !db_model_edges.is_empty() {
        db_model_edges.sort_by(|a, b| a.src.cmp(&b.src).then_with(|| a.model.cmp(&b.model)));
        let t_dbm = Instant::now();
        let dbm_count = db_model_edges.len();
        ok_chunks(&db_model_edges, CALLS_BATCH_SIZE)
            .try_for_each_concurrent(db_model_edge_concurrency, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.clone();
                async move { writers::write_db_model_edges(&g, chunk, &run_id).await }
            })
            .await?;
        eprintln!(
            "[ts-pack-index] CALLS_DB_MODEL writes done in {:.2}s (rows={})",
            t_dbm.elapsed().as_secs_f64(),
            dbm_count,
        );
    }

    if !external_api_urls.is_empty() && !external_api_edges.is_empty() {
        let t_ext = Instant::now();
        let mut external_nodes = Vec::new();
        for url in &external_api_urls {
            external_nodes.push(ExternalApiNode {
                id: external_api_id(project_id, url),
                url: url.clone(),
                project_id: Arc::clone(project_id),
            });
        }
        external_nodes.sort_by(|a, b| a.id.cmp(&b.id));
        ok_chunks(&external_nodes, NODE_BATCH_SIZE)
            .try_for_each_concurrent(NODE_CONCURRENCY, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.clone();
                async move { writers::write_external_api_nodes(&g, chunk, &run_id).await }
            })
            .await?;
        let mut external_api_edges = external_api_edges;
        external_api_edges.sort_by(|a, b| a.src.cmp(&b.src).then_with(|| a.tgt.cmp(&b.tgt)));
        let ext_count = external_api_edges.len();
        ok_chunks(&external_api_edges, CALLS_BATCH_SIZE)
            .try_for_each_concurrent(REL_CONCURRENCY, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.clone();
                async move { writers::write_external_api_edges(&g, chunk, &run_id).await }
            })
            .await?;
        eprintln!(
            "[ts-pack-index] CALLS_API_EXTERNAL writes done in {:.2}s (rows={})",
            t_ext.elapsed().as_secs_f64(),
            ext_count,
        );
    }

    if !file_import_edges.is_empty() {
        let mut file_import_edges = file_import_edges;
        file_import_edges.sort_by(|a, b| {
            a.src_filepath
                .cmp(&b.src_filepath)
                .then_with(|| a.tgt_filepath.cmp(&b.tgt_filepath))
        });
        let t_impf = Instant::now();
        let impf_count = file_import_edges.len();
        ok_chunks(&file_import_edges, CALLS_BATCH_SIZE)
            .try_for_each_concurrent(REL_CONCURRENCY, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.clone();
                async move { writers::write_file_import_edges(&g, chunk, &run_id).await }
            })
            .await?;
        eprintln!(
            "[ts-pack-index] IMPORTS writes done in {:.2}s (rows={})",
            t_impf.elapsed().as_secs_f64(),
            impf_count,
        );
    }

    if !asset_links.is_empty() {
        let mut asset_links = asset_links;
        asset_links.sort_by(|a, b| {
            a.src_filepath
                .cmp(&b.src_filepath)
                .then_with(|| a.tgt_filepath.cmp(&b.tgt_filepath))
        });
        ok_chunks(&asset_links, CALLS_BATCH_SIZE)
            .try_for_each_concurrent(REL_CONCURRENCY, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.clone();
                async move { writers::write_file_edges(&g, chunk, "ASSET_LINKS", &run_id).await }
            })
            .await?;
    }
    if !api_edges.is_empty() {
        let mut api_edges = api_edges;
        api_edges.sort_by(|a, b| {
            a.src_filepath
                .cmp(&b.src_filepath)
                .then_with(|| a.tgt_filepath.cmp(&b.tgt_filepath))
        });
        ok_chunks(&api_edges, CALLS_BATCH_SIZE)
            .try_for_each_concurrent(REL_CONCURRENCY, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.clone();
                async move { writers::write_file_edges(&g, chunk, "CALLS_API", &run_id).await }
            })
            .await?;
    }
    if !service_edges.is_empty() {
        let mut service_edges = service_edges;
        service_edges.sort_by(|a, b| {
            a.src_filepath
                .cmp(&b.src_filepath)
                .then_with(|| a.tgt_filepath.cmp(&b.tgt_filepath))
        });
        ok_chunks(&service_edges, CALLS_BATCH_SIZE)
            .try_for_each_concurrent(REL_CONCURRENCY, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.clone();
                async move { writers::write_file_edges(&g, chunk, "CALLS_SERVICE", &run_id).await }
            })
            .await?;
    }
    if !api_route_calls.is_empty() {
        ok_chunks(&api_route_calls, CALLS_BATCH_SIZE)
            .try_for_each_concurrent(REL_CONCURRENCY, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.clone();
                async move { writers::write_api_route_calls(&g, chunk, &run_id).await }
            })
            .await?;
    }
    if !api_route_handlers.is_empty() {
        ok_chunks(&api_route_handlers, CALLS_BATCH_SIZE)
            .try_for_each_concurrent(REL_CONCURRENCY, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.clone();
                async move { writers::write_api_route_handlers(&g, chunk, &run_id).await }
            })
            .await?;
    }
    if !resource_usages.is_empty() {
        let mut grouped: HashMap<String, Vec<ResourceUsageRow>> = HashMap::new();
        for row in resource_usages {
            grouped.entry(row.rel_name.clone()).or_default().push(row);
        }
        for (rel_name, rows) in grouped {
            ok_chunks(&rows, CALLS_BATCH_SIZE)
                .try_for_each_concurrent(REL_CONCURRENCY, |chunk| {
                    let g = Arc::clone(graph);
                    let rel_name = rel_name.clone();
                    let run_id = run_id.clone();
                    async move { writers::write_resource_usage_edges(&g, chunk, &rel_name, &run_id).await }
                })
                .await?;
        }
    }
    if !resource_backings.is_empty() {
        ok_chunks(&resource_backings, CALLS_BATCH_SIZE)
            .try_for_each_concurrent(REL_CONCURRENCY, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.clone();
                async move { writers::write_resource_backings(&g, chunk, &run_id).await }
            })
            .await?;
    }
    if !xcode_targets.is_empty() {
        ok_chunks(&xcode_targets, CALLS_BATCH_SIZE)
            .try_for_each_concurrent(REL_CONCURRENCY, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.clone();
                async move { writers::write_xcode_targets(&g, chunk, &run_id).await }
            })
            .await?;
    }
    if !cargo_crates.is_empty() {
        ok_chunks(&cargo_crates, CALLS_BATCH_SIZE)
            .try_for_each_concurrent(REL_CONCURRENCY, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.clone();
                async move { writers::write_cargo_crates(&g, chunk, &run_id).await }
            })
            .await?;
    }
    if !cargo_workspaces.is_empty() {
        ok_chunks(&cargo_workspaces, CALLS_BATCH_SIZE)
            .try_for_each_concurrent(REL_CONCURRENCY, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.clone();
                async move { writers::write_cargo_workspaces(&g, chunk, &run_id).await }
            })
            .await?;
    }
    if !cargo_workspace_crates.is_empty() {
        ok_chunks(&cargo_workspace_crates, CALLS_BATCH_SIZE)
            .try_for_each_concurrent(REL_CONCURRENCY, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.clone();
                async move { writers::write_cargo_workspace_crates(&g, chunk, &run_id).await }
            })
            .await?;
    }
    if !cargo_crate_files.is_empty() {
        ok_chunks(&cargo_crate_files, CALLS_BATCH_SIZE)
            .try_for_each_concurrent(REL_CONCURRENCY, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.clone();
                async move { writers::write_cargo_crate_files(&g, chunk, &run_id).await }
            })
            .await?;
    }
    if !cargo_dependency_edges.is_empty() {
        ok_chunks(&cargo_dependency_edges, CALLS_BATCH_SIZE)
            .try_for_each_concurrent(REL_CONCURRENCY, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.clone();
                async move { writers::write_cargo_dependency_edges(&g, chunk, &run_id).await }
            })
            .await?;
    }
    if !xcode_target_files.is_empty() {
        ok_chunks(&xcode_target_files, CALLS_BATCH_SIZE)
            .try_for_each_concurrent(REL_CONCURRENCY, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.clone();
                async move { writers::write_xcode_target_files(&g, chunk, &run_id).await }
            })
            .await?;
    }
    if !xcode_target_resources.is_empty() {
        ok_chunks(&xcode_target_resources, CALLS_BATCH_SIZE)
            .try_for_each_concurrent(REL_CONCURRENCY, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.clone();
                async move { writers::write_xcode_target_resources(&g, chunk, &run_id).await }
            })
            .await?;
    }
    if !xcode_workspaces.is_empty() {
        ok_chunks(&xcode_workspaces, CALLS_BATCH_SIZE)
            .try_for_each_concurrent(REL_CONCURRENCY, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.clone();
                async move { writers::write_xcode_workspaces(&g, chunk, &run_id).await }
            })
            .await?;
    }
    if !xcode_workspace_projects.is_empty() {
        ok_chunks(&xcode_workspace_projects, CALLS_BATCH_SIZE)
            .try_for_each_concurrent(REL_CONCURRENCY, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.clone();
                async move { writers::write_xcode_workspace_projects(&g, chunk, &run_id).await }
            })
            .await?;
    }
    if !xcode_schemes.is_empty() {
        ok_chunks(&xcode_schemes, CALLS_BATCH_SIZE)
            .try_for_each_concurrent(REL_CONCURRENCY, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.clone();
                async move { writers::write_xcode_schemes(&g, chunk, &run_id).await }
            })
            .await?;
    }
    if !xcode_scheme_targets.is_empty() {
        ok_chunks(&xcode_scheme_targets, CALLS_BATCH_SIZE)
            .try_for_each_concurrent(REL_CONCURRENCY, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.clone();
                async move { writers::write_xcode_scheme_targets(&g, chunk, &run_id).await }
            })
            .await?;
    }
    if !xcode_scheme_files.is_empty() {
        ok_chunks(&xcode_scheme_files, CALLS_BATCH_SIZE)
            .try_for_each_concurrent(REL_CONCURRENCY, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.clone();
                async move { writers::write_xcode_scheme_files(&g, chunk, &run_id).await }
            })
            .await?;
    }

    let t_nodes = Instant::now();
    let mut all_files = all_files;
    all_files.sort_by(|a, b| a.id.cmp(&b.id));
    ok_chunks(&all_files, NODE_BATCH_SIZE)
        .try_for_each_concurrent(NODE_CONCURRENCY, |chunk| {
            let g = Arc::clone(graph);
            let run_id = run_id.clone();
            async move { writers::write_file_nodes(&g, chunk, &run_id).await }
        })
        .await?;

    for nodes in all_symbols.values_mut() {
        nodes.sort_by(|a, b| a.id.cmp(&b.id));
    }
    let symbol_labels: Vec<(&'static str, Vec<SymbolNode>)> = all_symbols.into_iter().collect();
    for (label, nodes) in &symbol_labels {
        ok_chunks(nodes, NODE_BATCH_SIZE)
            .try_for_each_concurrent(NODE_CONCURRENCY, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.clone();
                async move { writers::write_symbol_nodes(&g, chunk, label, &run_id).await }
            })
            .await?;
    }

    let node_elapsed = t_nodes.elapsed();
    let total_symbols: usize = symbol_labels.iter().map(|(_, v)| v.len()).sum();
    eprintln!(
        "[ts-pack-index] Node writes done in {:.2}s (files={}, symbols={})",
        node_elapsed.as_secs_f64(),
        all_files.len(),
        total_symbols,
    );

    if !import_symbol_edges.is_empty() {
        let mut import_symbol_edges = import_symbol_edges;
        import_symbol_edges.sort_by(|a, b| a.src.cmp(&b.src).then_with(|| a.tgt.cmp(&b.tgt)));
        let t_imp = Instant::now();
        let imp_count = import_symbol_edges.len();
        ok_chunks(&import_symbol_edges, CALLS_BATCH_SIZE)
            .try_for_each_concurrent(symbol_edge_concurrency, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.clone();
                async move { writers::write_import_symbol_edges(&g, chunk, &run_id).await }
            })
            .await?;
        eprintln!(
            "[ts-pack-index] IMPORTS_SYMBOL writes done in {:.2}s (rows={})",
            t_imp.elapsed().as_secs_f64(),
            imp_count,
        );
    }

    if !implicit_import_symbol_edges.is_empty() {
        let mut implicit_import_symbol_edges = implicit_import_symbol_edges;
        implicit_import_symbol_edges.sort_by(|a, b| a.src.cmp(&b.src).then_with(|| a.tgt.cmp(&b.tgt)));
        let t_imp = Instant::now();
        let imp_count = implicit_import_symbol_edges.len();
        ok_chunks(&implicit_import_symbol_edges, CALLS_BATCH_SIZE)
            .try_for_each_concurrent(symbol_edge_concurrency, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.clone();
                async move { writers::write_implicit_import_symbol_edges(&g, chunk, &run_id).await }
            })
            .await?;
        eprintln!(
            "[ts-pack-index] IMPLICIT_IMPORTS_SYMBOL writes done in {:.2}s (rows={})",
            t_imp.elapsed().as_secs_f64(),
            imp_count,
        );
    }

    if !export_symbol_edges.is_empty() {
        let mut export_symbol_edges = export_symbol_edges;
        export_symbol_edges.sort_by(|a, b| a.src.cmp(&b.src).then_with(|| a.tgt.cmp(&b.tgt)));
        let t_exp = Instant::now();
        let exp_count = export_symbol_edges.len();
        ok_chunks(&export_symbol_edges, CALLS_BATCH_SIZE)
            .try_for_each_concurrent(symbol_edge_concurrency, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.clone();
                async move { writers::write_export_symbol_edges(&g, chunk, &run_id).await }
            })
            .await?;
        eprintln!(
            "[ts-pack-index] EXPORTS_SYMBOL writes done in {:.2}s (rows={})",
            t_exp.elapsed().as_secs_f64(),
            exp_count,
        );
    }
    if !export_alias_edges.is_empty() {
        let mut export_alias_edges = export_alias_edges;
        export_alias_edges.sort_by(|a, b| {
            a.src
                .cmp(&b.src)
                .then_with(|| a.tgt.cmp(&b.tgt))
                .then_with(|| a.exported_as.cmp(&b.exported_as))
        });
        let t_exp_alias = Instant::now();
        let exp_alias_count = export_alias_edges.len();
        ok_chunks(&export_alias_edges, CALLS_BATCH_SIZE)
            .try_for_each_concurrent(symbol_edge_concurrency, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.clone();
                async move { writers::write_export_alias_edges(&g, chunk, &run_id).await }
            })
            .await?;
        eprintln!(
            "[ts-pack-index] EXPORTS_SYMBOL_AS writes done in {:.2}s (rows={})",
            t_exp_alias.elapsed().as_secs_f64(),
            exp_alias_count,
        );
    }
    if !rust_impl_trait_edges.is_empty() {
        let mut rust_impl_trait_edges = rust_impl_trait_edges;
        rust_impl_trait_edges.sort_by(|a, b| a.impl_id.cmp(&b.impl_id).then_with(|| a.trait_name.cmp(&b.trait_name)));
        ok_chunks(&rust_impl_trait_edges, CALLS_BATCH_SIZE)
            .try_for_each_concurrent(symbol_edge_concurrency, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.clone();
                async move { writers::write_rust_impl_trait_edges(&g, chunk, &run_id).await }
            })
            .await?;
    }
    if !rust_impl_type_edges.is_empty() {
        let mut rust_impl_type_edges = rust_impl_type_edges;
        rust_impl_type_edges.sort_by(|a, b| a.impl_id.cmp(&b.impl_id).then_with(|| a.type_name.cmp(&b.type_name)));
        ok_chunks(&rust_impl_type_edges, CALLS_BATCH_SIZE)
            .try_for_each_concurrent(symbol_edge_concurrency, |chunk| {
                let g = Arc::clone(graph);
                let run_id = run_id.clone();
                async move { writers::write_rust_impl_type_edges(&g, chunk, &run_id).await }
            })
            .await?;
    }

    if !launch_edges.is_empty() {
        let mut launch_edges = launch_edges;
        launch_edges.sort_by(|a, b| {
            a.src_filepath
                .cmp(&b.src_filepath)
                .then_with(|| a.tgt_filepath.cmp(&b.tgt_filepath))
        });
        let t_launch = Instant::now();
        let launch_count = launch_edges.len();
        ok_chunks(&launch_edges, CALLS_BATCH_SIZE)
            .try_for_each_concurrent(REL_CONCURRENCY, |chunk| {
                let g = Arc::clone(graph);
                async move { writers::write_launch_edges(&g, chunk).await }
            })
            .await?;
        eprintln!(
            "[ts-pack-index] LAUNCHES writes done in {:.2}s (rows={})",
            t_launch.elapsed().as_secs_f64(),
            launch_count,
        );
    }

    let t_imports = Instant::now();
    let mut all_imports = all_imports;
    all_imports.sort_by(|a, b| a.id.cmp(&b.id));
    let import_count = all_imports.len();
    ok_chunks(&all_imports, IMPORT_BATCH_SIZE)
        .try_for_each_concurrent(NODE_CONCURRENCY, |chunk| {
            let g = Arc::clone(graph);
            let run_id = run_id.clone();
            async move { writers::write_import_nodes(&g, chunk, &run_id).await }
        })
        .await?;
    let import_elapsed = t_imports.elapsed();
    eprintln!(
        "[ts-pack-index] Import writes done in {:.2}s (count={})",
        import_elapsed.as_secs_f64(),
        import_count,
    );

    all_rels.extend(all_import_rels);
    all_rels.sort_by(|a, b| a.parent.cmp(&b.parent).then_with(|| a.child.cmp(&b.child)));
    let rel_count = all_rels.len();
    let t_rels = Instant::now();
    ok_chunks(&all_rels, REL_BATCH_SIZE)
        .try_for_each_concurrent(contains_rel_concurrency, |chunk| {
            let g = Arc::clone(graph);
            let run_id = run_id.clone();
            async move { writers::write_relationships(&g, chunk, &run_id).await }
        })
        .await?;
    let rel_elapsed = t_rels.elapsed();
    eprintln!(
        "[ts-pack-index] Relationship writes done in {:.2}s (count={})",
        rel_elapsed.as_secs_f64(),
        rel_count,
    );

    let t_calls = Instant::now();
    let mut all_symbol_call_rows = all_symbol_call_rows;
    all_symbol_call_rows.sort_by(|a, b| {
        a.caller_id
            .cmp(&b.caller_id)
            .then_with(|| a.callee.cmp(&b.callee))
            .then_with(|| a.caller_filepath.cmp(&b.caller_filepath))
    });
    let calls_row_count = all_symbol_call_rows.len();
    ok_chunks(&all_symbol_call_rows, CALL_EDGE_BATCH_SIZE)
        .try_for_each_concurrent(call_edge_concurrency, |chunk| {
            let g = Arc::clone(graph);
            let run_id = run_id.clone();
            async move { writers::write_calls(&g, chunk, &run_id).await }
        })
        .await?;

    let clone_enabled = std::env::var("LM_PROXY_CLONE_ENRICH")
        .ok()
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true);
    if clone_enabled && !clone_candidates.is_empty() {
        let cfg = clone_enrich::CloneConfig {
            min_overlap: crate::WINNOW_MIN_OVERLAP,
            token_sim_threshold: crate::WINNOW_TOKEN_SIM_THRESHOLD,
            kgram_sim_threshold: crate::WINNOW_KGRAM_SIM_THRESHOLD,
            min_score: crate::WINNOW_MIN_SCORE,
            bucket_limit: crate::WINNOW_BUCKET_LIMIT,
            fallback_hashes: crate::WINNOW_FALLBACK_HASHES,
            force_all_hashes_max_fps: crate::WINNOW_FORCE_ALL_HASHES_MAX_FPS,
        };
        clone_enrich::write_clone_enrichment(
            graph,
            project_id.as_ref(),
            &run_id,
            &clone_candidates,
            REL_BATCH_SIZE,
            REL_CONCURRENCY,
            &cfg,
        )
        .await?;
    }

    let calls_elapsed = t_calls.elapsed();
    eprintln!(
        "[ts-pack-index] CALLS writes done in {:.2}s (rows={})",
        calls_elapsed.as_secs_f64(),
        calls_row_count,
    );

    if !inferred_call_rows.is_empty() || !python_inferred_call_rows.is_empty() {
        let t_inf = Instant::now();
        let mut inferred_call_rows = inferred_call_rows;
        let mut python_inferred_call_rows = python_inferred_call_rows;
        inferred_call_rows.sort_by(|a, b| {
            a.caller_id
                .cmp(&b.caller_id)
                .then_with(|| a.receiver_type.cmp(&b.receiver_type))
                .then_with(|| a.callee.cmp(&b.callee))
        });
        python_inferred_call_rows.sort_by(|a, b| {
            a.caller_id
                .cmp(&b.caller_id)
                .then_with(|| a.callee_filepath.cmp(&b.callee_filepath))
                .then_with(|| a.callee.cmp(&b.callee))
        });
        let swift_count = inferred_call_rows.len();
        let py_count = python_inferred_call_rows.len();
        if !inferred_call_rows.is_empty() {
            ok_chunks(&inferred_call_rows, CALL_EDGE_BATCH_SIZE)
                .try_for_each_concurrent(call_edge_concurrency, |chunk| {
                    let g = Arc::clone(graph);
                    let run_id = run_id.clone();
                    async move { writers::write_inferred_calls(&g, chunk, &run_id).await }
                })
                .await?;
        }
        if !python_inferred_call_rows.is_empty() {
            ok_chunks(&python_inferred_call_rows, CALL_EDGE_BATCH_SIZE)
                .try_for_each_concurrent(call_edge_concurrency, |chunk| {
                    let g = Arc::clone(graph);
                    let run_id = run_id.clone();
                    async move { writers::write_python_inferred_calls(&g, chunk, &run_id).await }
                })
                .await?;
        }
        eprintln!(
            "[ts-pack-index] CALLS_INFERRED writes done in {:.2}s (rows={})",
            t_inf.elapsed().as_secs_f64(),
            swift_count + py_count,
        );
    }
    Ok(WritePhaseSummary {
        node_elapsed,
        import_elapsed,
        rel_elapsed,
        calls_elapsed,
    })
}
