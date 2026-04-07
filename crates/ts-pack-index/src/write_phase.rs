use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, Instant};

use futures::{StreamExt, stream};
use neo4rs::{Graph, Query};

use crate::clone_enrich;
use crate::writers;
use crate::{
    CALLS_BATCH_SIZE, CloneCandidate, DbEdgeRow, DbModelEdgeRow, ExportSymbolEdgeRow, ExternalApiEdgeRow,
    ExternalApiNode, FileImportEdgeRow, FileNode, IMPORT_BATCH_SIZE, ImplicitImportSymbolEdgeRow, ImportNode,
    ImportSymbolEdgeRow, InferredCallRow, LaunchEdgeRow, NODE_BATCH_SIZE, NODE_CONCURRENCY, PythonInferredCallRow,
    REL_BATCH_SIZE, REL_CONCURRENCY, RelRow, SymbolCallRow, SymbolNode, external_api_id, extract_prisma_models,
};

pub(crate) struct WritePhaseSummary {
    pub(crate) node_elapsed: Duration,
    pub(crate) import_elapsed: Duration,
    pub(crate) rel_elapsed: Duration,
    pub(crate) calls_elapsed: Duration,
}

pub(crate) struct WriteInputs {
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
    pub(crate) db_delegates_by_file: Vec<(String, String)>,
    pub(crate) external_api_edges: Vec<ExternalApiEdgeRow>,
    pub(crate) external_api_urls: HashSet<String>,
    pub(crate) file_import_edges: Vec<FileImportEdgeRow>,
    pub(crate) import_symbol_edges: Vec<ImportSymbolEdgeRow>,
    pub(crate) implicit_import_symbol_edges: Vec<ImplicitImportSymbolEdgeRow>,
    pub(crate) export_symbol_edges: Vec<ExportSymbolEdgeRow>,
    pub(crate) launch_edges: Vec<LaunchEdgeRow>,
    pub(crate) manifest_abs: HashMap<String, String>,
}

pub(crate) async fn run_write_phases(
    graph: &Arc<Graph>,
    project_id: &Arc<str>,
    inputs: WriteInputs,
) -> WritePhaseSummary {
    // Symbol-edge writes touch the same File/Node relationship groups heavily and
    // have proven prone to Neo4j deadlocks when batched concurrently.
    let symbol_edge_concurrency = 1usize;

    let WriteInputs {
        all_files,
        all_symbols,
        all_imports,
        mut all_rels,
        all_import_rels,
        all_symbol_call_rows,
        clone_candidates,
        inferred_call_rows,
        python_inferred_call_rows,
        db_sources,
        db_delegates_by_file,
        external_api_edges,
        external_api_urls,
        file_import_edges,
        import_symbol_edges,
        implicit_import_symbol_edges,
        export_symbol_edges,
        launch_edges,
        manifest_abs,
    } = inputs;

    let schema_id = all_files
        .iter()
        .find(|f| f.filepath == "prisma/schema.prisma")
        .map(|f| f.id.clone());
    if let Some(schema_id) = schema_id {
        writers::run_query_logged(
            graph,
            Query::new("MATCH (:File {project_id: $pid})-[r:CALLS_DB]->() DELETE r".to_string())
                .param("pid", project_id.to_string()),
            "delete_calls_db",
        )
        .await;
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
            let t_db = Instant::now();
            let db_count = db_edges.len();
            stream::iter(db_edges.chunks(CALLS_BATCH_SIZE))
                .for_each_concurrent(REL_CONCURRENCY, |chunk| {
                    let g = Arc::clone(graph);
                    async move { writers::write_db_edges(&g, chunk).await }
                })
                .await;
            eprintln!(
                "[ts-pack-index] CALLS_DB writes done in {:.2}s (rows={})",
                t_db.elapsed().as_secs_f64(),
                db_count,
            );
        }

        if let Some(schema_abs) = manifest_abs.get("prisma/schema.prisma") {
            if let Ok(schema_text) = std::fs::read_to_string(schema_abs) {
                let mut model_map: HashMap<String, String> = HashMap::new();
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

                let mut db_model_edges = Vec::new();
                for (file_id, delegate) in &db_delegates_by_file {
                    if let Some(model) = model_map.get(&delegate.to_lowercase()) {
                        db_model_edges.push(DbModelEdgeRow {
                            src: file_id.clone(),
                            model: model.clone(),
                            project_id: Arc::clone(project_id),
                        });
                    }
                }

                writers::run_query_logged(
                    graph,
                    Query::new("MATCH (m:Model {project_id: $pid}) DETACH DELETE m".to_string())
                        .param("pid", project_id.to_string()),
                    "delete_models",
                )
                .await;
                if !db_model_edges.is_empty() {
                    let t_dbm = Instant::now();
                    let dbm_count = db_model_edges.len();
                    stream::iter(db_model_edges.chunks(CALLS_BATCH_SIZE))
                        .for_each_concurrent(REL_CONCURRENCY, |chunk| {
                            let g = Arc::clone(graph);
                            async move { writers::write_db_model_edges(&g, chunk).await }
                        })
                        .await;
                    eprintln!(
                        "[ts-pack-index] CALLS_DB_MODEL writes done in {:.2}s (rows={})",
                        t_dbm.elapsed().as_secs_f64(),
                        dbm_count,
                    );
                }
            }
        }
    }

    writers::run_query_logged(
        graph,
        Query::new("MATCH (:File {project_id: $pid})-[r:CALLS_API_EXTERNAL]->() DELETE r".to_string())
            .param("pid", project_id.to_string()),
        "delete_calls_api_external",
    )
    .await;
    writers::run_query_logged(
        graph,
        Query::new("MATCH (e:ExternalAPI {project_id: $pid}) DETACH DELETE e".to_string())
            .param("pid", project_id.to_string()),
        "delete_external_api_nodes",
    )
    .await;
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
        stream::iter(external_nodes.chunks(NODE_BATCH_SIZE))
            .for_each_concurrent(NODE_CONCURRENCY, |chunk| {
                let g = Arc::clone(graph);
                async move { writers::write_external_api_nodes(&g, chunk).await }
            })
            .await;
        let ext_count = external_api_edges.len();
        stream::iter(external_api_edges.chunks(CALLS_BATCH_SIZE))
            .for_each_concurrent(REL_CONCURRENCY, |chunk| {
                let g = Arc::clone(graph);
                async move { writers::write_external_api_edges(&g, chunk).await }
            })
            .await;
        eprintln!(
            "[ts-pack-index] CALLS_API_EXTERNAL writes done in {:.2}s (rows={})",
            t_ext.elapsed().as_secs_f64(),
            ext_count,
        );
    }

    writers::run_query_logged(
        graph,
        Query::new("MATCH (:File {project_id: $pid})-[r:IMPORTS]->() DELETE r".to_string())
            .param("pid", project_id.to_string()),
        "delete_imports",
    )
    .await;
    if !file_import_edges.is_empty() {
        let t_impf = Instant::now();
        let impf_count = file_import_edges.len();
        stream::iter(file_import_edges.chunks(CALLS_BATCH_SIZE))
            .for_each_concurrent(REL_CONCURRENCY, |chunk| {
                let g = Arc::clone(graph);
                async move { writers::write_file_import_edges(&g, chunk).await }
            })
            .await;
        eprintln!(
            "[ts-pack-index] IMPORTS writes done in {:.2}s (rows={})",
            t_impf.elapsed().as_secs_f64(),
            impf_count,
        );
    }

    writers::run_query_logged(
        graph,
        Query::new("MATCH (:File {project_id: $pid})-[r:IMPORTS_SYMBOL]->() DELETE r".to_string())
            .param("pid", project_id.to_string()),
        "delete_imports_symbol",
    )
    .await;
    writers::run_query_logged(
        graph,
        Query::new("MATCH (:File {project_id: $pid})-[r:IMPLICIT_IMPORTS_SYMBOL]->() DELETE r".to_string())
            .param("pid", project_id.to_string()),
        "delete_implicit_imports_symbol",
    )
    .await;
    writers::run_query_logged(
        graph,
        Query::new("MATCH (:File {project_id: $pid})-[r:EXPORTS_SYMBOL]->() DELETE r".to_string())
            .param("pid", project_id.to_string()),
        "delete_exports_symbol",
    )
    .await;

    let t_nodes = Instant::now();
    stream::iter(all_files.chunks(NODE_BATCH_SIZE))
        .for_each_concurrent(NODE_CONCURRENCY, |chunk| {
            let g = Arc::clone(graph);
            async move { writers::write_file_nodes(&g, chunk).await }
        })
        .await;

    let symbol_labels: Vec<(&'static str, Vec<SymbolNode>)> = all_symbols.into_iter().collect();
    for (label, nodes) in &symbol_labels {
        stream::iter(nodes.chunks(NODE_BATCH_SIZE))
            .for_each_concurrent(NODE_CONCURRENCY, |chunk| {
                let g = Arc::clone(graph);
                async move { writers::write_symbol_nodes(&g, chunk, label).await }
            })
            .await;
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
        let t_imp = Instant::now();
        let imp_count = import_symbol_edges.len();
        stream::iter(import_symbol_edges.chunks(CALLS_BATCH_SIZE))
            .for_each_concurrent(symbol_edge_concurrency, |chunk| {
                let g = Arc::clone(graph);
                async move { writers::write_import_symbol_edges(&g, chunk).await }
            })
            .await;
        eprintln!(
            "[ts-pack-index] IMPORTS_SYMBOL writes done in {:.2}s (rows={})",
            t_imp.elapsed().as_secs_f64(),
            imp_count,
        );
    }

    if !implicit_import_symbol_edges.is_empty() {
        let t_imp = Instant::now();
        let imp_count = implicit_import_symbol_edges.len();
        stream::iter(implicit_import_symbol_edges.chunks(CALLS_BATCH_SIZE))
            .for_each_concurrent(symbol_edge_concurrency, |chunk| {
                let g = Arc::clone(graph);
                async move { writers::write_implicit_import_symbol_edges(&g, chunk).await }
            })
            .await;
        eprintln!(
            "[ts-pack-index] IMPLICIT_IMPORTS_SYMBOL writes done in {:.2}s (rows={})",
            t_imp.elapsed().as_secs_f64(),
            imp_count,
        );
    }

    if !export_symbol_edges.is_empty() {
        let t_exp = Instant::now();
        let exp_count = export_symbol_edges.len();
        stream::iter(export_symbol_edges.chunks(CALLS_BATCH_SIZE))
            .for_each_concurrent(symbol_edge_concurrency, |chunk| {
                let g = Arc::clone(graph);
                async move { writers::write_export_symbol_edges(&g, chunk).await }
            })
            .await;
        eprintln!(
            "[ts-pack-index] EXPORTS_SYMBOL writes done in {:.2}s (rows={})",
            t_exp.elapsed().as_secs_f64(),
            exp_count,
        );
    }

    if !launch_edges.is_empty() {
        let t_launch = Instant::now();
        let launch_count = launch_edges.len();
        stream::iter(launch_edges.chunks(CALLS_BATCH_SIZE))
            .for_each_concurrent(REL_CONCURRENCY, |chunk| {
                let g = Arc::clone(graph);
                async move { writers::write_launch_edges(&g, chunk).await }
            })
            .await;
        eprintln!(
            "[ts-pack-index] LAUNCHES writes done in {:.2}s (rows={})",
            t_launch.elapsed().as_secs_f64(),
            launch_count,
        );
    }

    let t_imports = Instant::now();
    let import_count = all_imports.len();
    stream::iter(all_imports.chunks(IMPORT_BATCH_SIZE))
        .for_each_concurrent(NODE_CONCURRENCY, |chunk| {
            let g = Arc::clone(graph);
            async move { writers::write_import_nodes(&g, chunk).await }
        })
        .await;
    let import_elapsed = t_imports.elapsed();
    eprintln!(
        "[ts-pack-index] Import writes done in {:.2}s (count={})",
        import_elapsed.as_secs_f64(),
        import_count,
    );

    all_rels.extend(all_import_rels);
    let rel_count = all_rels.len();
    let t_rels = Instant::now();
    stream::iter(all_rels.chunks(REL_BATCH_SIZE))
        .for_each_concurrent(REL_CONCURRENCY, |chunk| {
            let g = Arc::clone(graph);
            async move { writers::write_relationships(&g, chunk).await }
        })
        .await;
    let rel_elapsed = t_rels.elapsed();
    eprintln!(
        "[ts-pack-index] Relationship writes done in {:.2}s (count={})",
        rel_elapsed.as_secs_f64(),
        rel_count,
    );

    let t_calls = Instant::now();
    let calls_row_count = all_symbol_call_rows.len();
    stream::iter(all_symbol_call_rows.chunks(CALLS_BATCH_SIZE))
        .for_each_concurrent(REL_CONCURRENCY, |chunk| {
            let g = Arc::clone(graph);
            async move { writers::write_calls(&g, chunk).await }
        })
        .await;

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
            &clone_candidates,
            REL_BATCH_SIZE,
            REL_CONCURRENCY,
            &cfg,
        )
        .await;
    }

    let calls_elapsed = t_calls.elapsed();
    eprintln!(
        "[ts-pack-index] CALLS writes done in {:.2}s (rows={})",
        calls_elapsed.as_secs_f64(),
        calls_row_count,
    );

    if !inferred_call_rows.is_empty() || !python_inferred_call_rows.is_empty() {
        let t_inf = Instant::now();
        let swift_count = inferred_call_rows.len();
        let py_count = python_inferred_call_rows.len();
        if !inferred_call_rows.is_empty() {
            stream::iter(inferred_call_rows.chunks(CALLS_BATCH_SIZE))
                .for_each_concurrent(REL_CONCURRENCY, |chunk| {
                    let g = Arc::clone(graph);
                    async move { writers::write_inferred_calls(&g, chunk).await }
                })
                .await;
        }
        if !python_inferred_call_rows.is_empty() {
            stream::iter(python_inferred_call_rows.chunks(CALLS_BATCH_SIZE))
                .for_each_concurrent(REL_CONCURRENCY, |chunk| {
                    let g = Arc::clone(graph);
                    async move { writers::write_python_inferred_calls(&g, chunk).await }
                })
                .await;
        }
        eprintln!(
            "[ts-pack-index] CALLS_INFERRED writes done in {:.2}s (rows={})",
            t_inf.elapsed().as_secs_f64(),
            swift_count + py_count,
        );
    }
    WritePhaseSummary {
        node_elapsed,
        import_elapsed,
        rel_elapsed,
        calls_elapsed,
    }
}
