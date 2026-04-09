mod asset_phase;
mod clone_enrich;
pub mod duplicate;
mod model;
mod parse_phase;
mod pathing;
mod prep_phase;
mod swift;
mod tags;
mod write_phase;
mod writers;

use neo4rs::{Graph, Query};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;
use tree_sitter_language_pack as ts_pack;

pub(crate) use model::*;

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ManifestEntry {
    pub abs_path: String,
    pub rel_path: String,
    pub ext: String,
    pub size: u64,
}

pub struct IndexerConfig {
    pub neo4j_uri: String,
    pub neo4j_user: String,
    pub neo4j_pass: String,
    pub project_id: String,
    pub manifest_file: Option<String>,
}

// ---------------------------------------------------------------------------
// Tuning constants — one set, used throughout
// ---------------------------------------------------------------------------

/// Number of nodes per UNWIND batch (file nodes, symbol nodes)
pub(crate) const NODE_BATCH_SIZE: usize = 5000;
/// Number of :CONTAINS / :IMPORTS relationships per UNWIND batch.
/// Smaller than nodes: each row acquires 2 read locks + 1 write scan.
pub(crate) const REL_BATCH_SIZE: usize = 1000;
/// Number of Import nodes per UNWIND batch
pub(crate) const IMPORT_BATCH_SIZE: usize = 2000;
/// Number of CallSiteRow items per CALLS write batch (each may unwind many callees)
pub(crate) const CALLS_BATCH_SIZE: usize = 200;

/// Concurrent writers for node phases.
/// Neo4j Community Edition serialises above 4 concurrent writers internally;
/// higher values just add connection + lock-queue overhead.
pub(crate) const NODE_CONCURRENCY: usize = 4;

/// Concurrent writers for relationship MERGE.
/// Relationship MERGE scans all edges from the parent node (O(degree)).
/// 2 is the sweet spot on Community Edition for this query shape.
pub(crate) const REL_CONCURRENCY: usize = 2;

/// Max files processed in one Rayon + Neo4j cycle before writing
const MANIFEST_BATCH_SIZE: usize = 1000;

/// Max source file size: skip files larger than 1 MB
pub(crate) const MAX_FILE_BYTES: usize = 1_000_000;

// ---------------------------------------------------------------------------
// Clone grouping (winnow) defaults
// ---------------------------------------------------------------------------

pub(crate) const WINNOW_MIN_TOKENS: usize = 20;
pub(crate) const WINNOW_MIN_FINGERPRINTS: usize = 12;
pub(crate) const WINNOW_BUCKET_LIMIT: usize = 40;
pub(crate) const WINNOW_FALLBACK_HASHES: usize = 6;
pub(crate) const WINNOW_FORCE_ALL_HASHES_MAX_FPS: usize = 25;
pub(crate) const WINNOW_MIN_OVERLAP: f64 = 0.6;
pub(crate) const WINNOW_TOKEN_SIM_THRESHOLD: f64 = 0.65;
pub(crate) const WINNOW_KGRAM_SIM_THRESHOLD: f64 = 0.7;
pub(crate) const WINNOW_MIN_SCORE: f64 = 0.85;
pub(crate) const WINNOW_SMALL_TOKEN_THRESHOLD: usize = 50;

pub(crate) const WINNOW_SMALL_K: usize = 5;
pub(crate) const WINNOW_SMALL_W: usize = 3;
pub(crate) const WINNOW_MEDIUM_K: usize = 9;
pub(crate) const WINNOW_MEDIUM_W: usize = 5;
pub(crate) const WINNOW_LARGE_K: usize = 15;
pub(crate) const WINNOW_LARGE_W: usize = 7;

pub(crate) fn external_api_id(project_id: &str, url: &str) -> String {
    pathing::external_api_id(project_id, url)
}

fn project_root_from_manifest(manifest: &[ManifestEntry]) -> Option<String> {
    pathing::project_root_from_manifest(manifest)
}

pub(crate) fn extract_prisma_models(schema_text: &str) -> Vec<String> {
    let mut models: HashSet<String> = HashSet::new();
    let tree = match ts_pack::parse_string("prisma", schema_text.as_bytes()) {
        Ok(tree) => tree,
        Err(_) => return Vec::new(),
    };
    let matches = match ts_pack::run_query(
        &tree,
        "prisma",
        "(model_block (identifier) @model)",
        schema_text.as_bytes(),
    ) {
        Ok(m) => m,
        Err(_) => return Vec::new(),
    };
    for m in matches {
        for (cap, node_info) in m.captures {
            if cap != "model" {
                continue;
            }
            if let Ok(text) = ts_pack::extract_text(schema_text.as_bytes(), &node_info) {
                let name = text.trim();
                if !name.is_empty() {
                    models.insert(name.to_string());
                }
            }
        }
    }
    models.into_iter().collect()
}

// ---------------------------------------------------------------------------
// Main entry point
// ---------------------------------------------------------------------------

pub async fn index_workspace(
    _root_path: &Path,
    config: IndexerConfig,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let t0 = Instant::now();
    let run_id = format!("{}:{}", config.project_id, t0.elapsed().as_nanos());

    if let Ok(dir) = std::env::var("TS_PACK_CACHE_DIR").or_else(|_| std::env::var("LM_PROXY_TS_PACK_CACHE_DIR")) {
        if !dir.trim().is_empty() {
            let _ = ts_pack::configure(&ts_pack::PackConfig {
                cache_dir: Some(PathBuf::from(dir)),
                languages: None,
                groups: None,
            });
        }
    }

    // --- Neo4j connection ------------------------------------------------
    let neo4j_config = neo4rs::ConfigBuilder::default()
        .uri(&config.neo4j_uri)
        .user(&config.neo4j_user)
        .password(&config.neo4j_pass)
        .db("proxy")
        .max_connections(16)
        .fetch_size(500)
        .build()?;

    let graph = Arc::new(Graph::connect(neo4j_config).await?);

    // Schema setup: identity constraint + relationship index.
    // The CONTAINS index lets Neo4j short-circuit the edge scan in rel MERGE.
    for ddl in &[
        "CREATE CONSTRAINT node_id_unique IF NOT EXISTS FOR (n:Node) REQUIRE n.id IS UNIQUE",
        "CREATE INDEX contains_idx IF NOT EXISTS FOR ()-[r:CONTAINS]-() ON (r.project_id)",
    ] {
        writers::run_query_logged(&graph, Query::new(ddl.to_string()), "schema_ddl").await?;
    }

    // --- Load manifest ----------------------------------------------------
    let manifest: Vec<ManifestEntry> = match &config.manifest_file {
        Some(path) => {
            let content = std::fs::read_to_string(path)?;
            serde_json::from_str(&content)?
        }
        None => return Err("Manifest file required for indexing".into()),
    };

    let project_root = project_root_from_manifest(&manifest);

    let total_files = manifest.len();
    let project_id: Arc<str> = Arc::from(config.project_id.as_str());

    eprintln!("[ts-pack-index] Starting — {total_files} files in manifest");

    // --- Global data reservoirs ------------------------------------------
    let mut all_files: Vec<FileNode> = Vec::with_capacity(total_files);
    let mut all_symbols: HashMap<&'static str, Vec<SymbolNode>> = HashMap::new();
    let mut all_rels: Vec<RelRow> = Vec::new();
    let mut all_imports: Vec<ImportNode> = Vec::new();
    let mut all_import_rels: Vec<RelRow> = Vec::new();
    let mut all_symbol_call_rows: Vec<SymbolCallRow> = Vec::new();
    let mut inferred_call_rows: Vec<InferredCallRow> = Vec::new();
    let mut python_inferred_call_rows: Vec<PythonInferredCallRow> = Vec::new();
    let mut clone_candidates: Vec<CloneCandidate> = Vec::new();
    let mut swift_extension_map: std::collections::HashMap<String, std::collections::HashSet<String>> =
        std::collections::HashMap::new();
    let mut swift_contexts: Vec<SwiftFileContext> = Vec::new();
    let mut python_contexts: Vec<PythonFileContext> = Vec::new();
    let mut db_sources: Vec<String> = Vec::new();
    let mut db_model_refs_by_file: Vec<(String, String)> = Vec::new();
    let mut external_api_edges: Vec<ExternalApiEdgeRow> = Vec::new();
    let mut external_api_urls: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut seen_external_edges: std::collections::HashSet<(String, String)> = std::collections::HashSet::new();
    let mut import_symbol_edges: Vec<ImportSymbolEdgeRow> = Vec::new();
    let mut implicit_import_symbol_edges: Vec<ImplicitImportSymbolEdgeRow> = Vec::new();
    let mut export_symbol_edges: Vec<ExportSymbolEdgeRow> = Vec::new();
    let mut export_alias_edges: Vec<ExportAliasEdgeRow> = Vec::new();
    let mut launch_requests: Vec<(String, String)> = Vec::new();
    let mut seen_export_symbol: std::collections::HashSet<(String, String)> = std::collections::HashSet::new();
    let mut import_symbol_requests: Vec<ImportSymbolRequest> = Vec::new();
    let mut reexport_symbol_requests: Vec<ReExportSymbolRequest> = Vec::new();
    let mut export_alias_requests: Vec<ExportAliasRequest> = Vec::new();
    let mut all_file_facts: HashMap<String, ts_pack::FileFacts> = HashMap::new();
    let timing_enabled = std::env::var("TS_PACK_DEBUG_PARSE_TIMINGS")
        .ok()
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(false);
    let mut parse_tree_total = 0.0f64;
    let mut file_facts_total = 0.0f64;
    let mut process_total = 0.0f64;
    let mut tags_total = 0.0f64;
    let mut tags_total_by_lang: HashMap<String, (f64, usize)> = HashMap::new();
    let query_profile_enabled = std::env::var("TS_PACK_DEBUG_QUERY_PROFILE")
        .ok()
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(false);
    if query_profile_enabled {
        tags::reset_query_profile_aggregates();
    }

    // --- Phase 1: Parse files in parallel batches ------------------------
    let t_parse = Instant::now();
    let mut files_parsed = 0usize;

    for (batch_idx, batch) in manifest.chunks(MANIFEST_BATCH_SIZE).enumerate() {
        let batch_start = batch_idx * MANIFEST_BATCH_SIZE;
        eprintln!(
            "[ts-pack-index] Parsing batch {}/{} (files {}-{})",
            batch_idx + 1,
            (total_files + MANIFEST_BATCH_SIZE - 1) / MANIFEST_BATCH_SIZE,
            batch_start,
            batch_start + batch.len(),
        );

        let batch_results = parse_phase::parse_manifest_batch(batch, Arc::clone(&project_id));

        // Merge batch results into global reservoirs
        for res in batch_results {
            let file_id = res.file_node.id.clone();
            let file_fp = res.file_node.filepath.clone();
            if timing_enabled {
                parse_tree_total += res.timings.parse_tree_secs;
                file_facts_total += res.timings.file_facts_secs;
                process_total += res.timings.process_secs;
                tags_total += res.timings.tags_secs;
                let entry = tags_total_by_lang.entry(res.language.clone()).or_insert((0.0, 0));
                entry.0 += res.timings.tags_secs;
                entry.1 += 1;
            }
            all_symbol_call_rows.extend(res.symbol_calls);
            all_file_facts.insert(file_fp.clone(), res.file_facts);
            all_files.push(res.file_node);
            let local_symbols = res.symbols;
            if !local_symbols.is_empty() {
                for syms in local_symbols.values() {
                    for sym in syms {
                        if sym.is_exported {
                            if seen_export_symbol.insert((file_id.clone(), sym.id.clone())) {
                                export_symbol_edges.push(ExportSymbolEdgeRow {
                                    src: file_id.clone(),
                                    tgt: sym.id.clone(),
                                });
                            }
                        }
                    }
                }
            }
            for (label, syms) in local_symbols {
                all_symbols.entry(label).or_default().extend(syms);
            }
            all_rels.extend(res.relations);
            all_imports.extend(res.imports);
            all_import_rels.extend(res.import_rels);
            if !res.db_models.is_empty() {
                db_sources.push(file_id.clone());
                for name in res.db_models {
                    db_model_refs_by_file.push((file_id.clone(), name));
                }
            }
            if !res.external_urls.is_empty() {
                for url in res.external_urls {
                    let external_id = external_api_id(&project_id, &url);
                    external_api_urls.insert(url.clone());
                    if seen_external_edges.insert((file_id.clone(), external_id.clone())) {
                        external_api_edges.push(ExternalApiEdgeRow {
                            src: file_id.clone(),
                            tgt: external_id,
                        });
                    }
                }
            }
            if !res.import_symbol_requests.is_empty() {
                import_symbol_requests.extend(res.import_symbol_requests);
            }
            if !res.reexport_symbol_requests.is_empty() {
                reexport_symbol_requests.extend(res.reexport_symbol_requests);
            }
            if !res.export_alias_requests.is_empty() {
                export_alias_requests.extend(res.export_alias_requests);
            }
            if !res.launch_calls.is_empty() {
                for target in res.launch_calls {
                    launch_requests.push((file_fp.clone(), target));
                }
            }
            if let Some(exts) = res.swift_extensions {
                for (ty, methods) in exts {
                    swift_extension_map.entry(ty).or_default().extend(methods);
                }
            }
            if let Some(ctx) = res.swift_context {
                swift_contexts.push(ctx);
            }
            if let Some(ctx) = res.python_context {
                python_contexts.push(ctx);
            }
            if !res.clone_candidates.is_empty() {
                clone_candidates.extend(res.clone_candidates);
            }
        }

        files_parsed += batch.len();
    }

    let parse_elapsed = t_parse.elapsed();
    eprintln!(
        "[ts-pack-index] Parse complete — {files_parsed} files in {:.2}s | \
         files={} symbols={} rels={} imports={}",
        parse_elapsed.as_secs_f64(),
        all_files.len(),
        all_symbols.values().map(|v| v.len()).sum::<usize>(),
        all_rels.len(),
        all_imports.len(),
    );
    if timing_enabled && files_parsed > 0 {
        eprintln!(
            "[ts-pack-index] Parse timings — parse_tree={:.2}s file_facts={:.2}s process={:.2}s tags={:.2}s | per_file_ms parse={:.2} facts={:.2} process={:.2} tags={:.2}",
            parse_tree_total,
            file_facts_total,
            process_total,
            tags_total,
            (parse_tree_total * 1000.0) / files_parsed as f64,
            (file_facts_total * 1000.0) / files_parsed as f64,
            (process_total * 1000.0) / files_parsed as f64,
            (tags_total * 1000.0) / files_parsed as f64,
        );
        let mut lang_rows: Vec<_> = tags_total_by_lang.into_iter().collect();
        lang_rows.sort_by(|a, b| b.1.0.partial_cmp(&a.1.0).unwrap_or(std::cmp::Ordering::Equal));
        let lang_summary = lang_rows
            .into_iter()
            .take(8)
            .map(|(lang, (secs, files))| {
                let per_file_ms = if files == 0 {
                    0.0
                } else {
                    (secs * 1000.0) / files as f64
                };
                format!("{lang}={secs:.2}s/{per_file_ms:.2}ms ({files} files)")
            })
            .collect::<Vec<_>>()
            .join(" ");
        if !lang_summary.is_empty() {
            eprintln!("[ts-pack-index] Tags by language — {lang_summary}");
        }
        if query_profile_enabled {
            let (mut by_label, mut by_file) = tags::summarize_query_profile_aggregates();
            by_label.sort_by(|a, b| {
                b.total_elapsed_secs
                    .partial_cmp(&a.total_elapsed_secs)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            by_file.sort_by(|a, b| {
                b.total_elapsed_secs
                    .partial_cmp(&a.total_elapsed_secs)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            let label_summary = by_label
                .into_iter()
                .take(8)
                .map(|row| {
                    format!(
                        "{}:{}={:.2}s/{}runs max={:.2}ms matches={} max_matches={} limit_hits={}",
                        row.lang,
                        row.label,
                        row.total_elapsed_secs,
                        row.runs,
                        row.max_elapsed_secs * 1000.0,
                        row.total_matches,
                        row.max_matches,
                        row.exceeded_match_limit_count,
                    )
                })
                .collect::<Vec<_>>()
                .join(" ");
            if !label_summary.is_empty() {
                eprintln!("[ts-pack-index] Query profile by label — {label_summary}");
            }
            let file_summary = by_file
                .into_iter()
                .take(8)
                .map(|row| {
                    format!(
                        "{}:{}:{}={:.2}s/{}runs max={:.2}ms matches={} max_matches={} limit_hits={}",
                        row.lang,
                        row.label,
                        row.file_path.unwrap_or_default(),
                        row.total_elapsed_secs,
                        row.runs,
                        row.max_elapsed_secs * 1000.0,
                        row.total_matches,
                        row.max_matches,
                        row.exceeded_match_limit_count,
                    )
                })
                .collect::<Vec<_>>()
                .join(" ");
            if !file_summary.is_empty() {
                eprintln!("[ts-pack-index] Query profile by file — {file_summary}");
            }
        }
    }

    let mut manifest_abs: HashMap<String, String> = HashMap::new();
    for entry in &manifest {
        manifest_abs.insert(entry.rel_path.clone(), entry.abs_path.clone());
    }

    let prep = prep_phase::prepare_graph_facts(
        &all_symbols,
        &all_files,
        &project_id,
        project_root.as_deref(),
        &manifest_abs,
        &all_file_facts,
        &launch_requests,
        &import_symbol_requests,
        &reexport_symbol_requests,
        &export_alias_requests,
        &swift_extension_map,
        &swift_contexts,
        &python_contexts,
    );
    import_symbol_edges.extend(prep.import_symbol_edges);
    export_symbol_edges.extend(prep.export_symbol_edges);
    export_alias_edges.extend(prep.export_alias_edges);
    implicit_import_symbol_edges.extend(prep.implicit_import_symbol_edges);
    inferred_call_rows.extend(prep.inferred_call_rows);
    python_inferred_call_rows.extend(prep.python_inferred_call_rows);
    let file_import_edges = prep.file_import_edges;
    let launch_edges = prep.launch_edges;

    let write_summary = write_phase::run_write_phases(
        &graph,
        &project_id,
        write_phase::WriteInputs {
            all_files,
            all_symbols,
            all_imports,
            all_rels,
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
            asset_links: prep.asset_links,
            api_edges: prep.api_edges,
            api_route_calls: prep.api_route_calls,
            api_route_handlers: prep.api_route_handlers,
            service_edges: prep.service_edges,
            resource_usages: prep.resource_usages,
            resource_backings: prep.resource_backings,
            xcode_targets: prep.xcode_targets,
            xcode_target_files: prep.xcode_target_files,
            xcode_target_resources: prep.xcode_target_resources,
            xcode_workspaces: prep.xcode_workspaces,
            xcode_workspace_projects: prep.xcode_workspace_projects,
            xcode_schemes: prep.xcode_schemes,
            xcode_scheme_targets: prep.xcode_scheme_targets,
            xcode_scheme_files: prep.xcode_scheme_files,
            cargo_crates: prep.cargo_crates,
            cargo_workspaces: prep.cargo_workspaces,
            cargo_workspace_crates: prep.cargo_workspace_crates,
            cargo_crate_files: prep.cargo_crate_files,
            cargo_dependency_edges: prep.cargo_dependency_edges,
            import_symbol_edges,
            implicit_import_symbol_edges,
            rust_impl_trait_edges: prep.rust_impl_trait_edges,
            rust_impl_type_edges: prep.rust_impl_type_edges,
            export_symbol_edges,
            export_alias_edges,
            launch_edges,
            manifest_abs,
        },
    )
    .await?;

    // --- Summary ----------------------------------------------------------
    let total_elapsed = t0.elapsed();
    eprintln!(
        "[ts-pack-index] Done — {total_files} files | \
         parse={:.2}s nodes={:.2}s imports={:.2}s rels={:.2}s calls={:.2}s total={:.2}s",
        parse_elapsed.as_secs_f64(),
        write_summary.node_elapsed.as_secs_f64(),
        write_summary.import_elapsed.as_secs_f64(),
        write_summary.rel_elapsed.as_secs_f64(),
        write_summary.calls_elapsed.as_secs_f64(),
        total_elapsed.as_secs_f64(),
    );

    writers::run_query_logged(
        &graph,
        Query::new(
            "MERGE (r:IndexRun {id:$id}) \
             SET r.project_id = $pid, \
                 r.status = 'done', \
                 r.finished_at = timestamp()"
                .to_string(),
        )
        .param("id", run_id)
        .param("pid", config.project_id.to_string()),
        "index_run_complete",
    )
    .await?;

    let indexed_paths: Vec<PathBuf> = manifest.into_iter().map(|m| PathBuf::from(m.abs_path)).collect();

    Ok(indexed_paths)
}
