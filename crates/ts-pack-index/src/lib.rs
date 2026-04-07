mod clone_enrich;
mod pathing;
mod parse_phase;
mod prep_phase;
mod swift;
mod tags;
mod write_phase;
mod writers;

use neo4rs::{Graph, Query};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;
use tree_sitter_language_pack as ts_pack;

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
// Typed payload structs (avoids serde_json::json! round-trips in hot loops)
// ---------------------------------------------------------------------------

pub(crate) struct FileNode {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) filepath: String,
    pub(crate) project_id: Arc<str>,
}

pub(crate) struct SymbolNode {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) kind: String,
    pub(crate) qualified_name: Option<String>,
    pub(crate) filepath: String,
    pub(crate) project_id: Arc<str>,
    pub(crate) start_line: u32,
    pub(crate) end_line: u32,
    pub(crate) start_byte: usize,
    pub(crate) end_byte: usize,
    pub(crate) signature: Option<String>,
    pub(crate) visibility: Option<String>,
    pub(crate) is_exported: bool,
    pub(crate) doc_comment: Option<String>,
}

pub(crate) struct RelRow {
    pub(crate) parent: String,
    pub(crate) child: String,
}

/// One resolved call edge: caller is a Symbol (or File as fallback) → callee symbol name.
pub(crate) struct SymbolCallRow {
    pub(crate) caller_id: String, // id of the calling Symbol node (or File if at file scope)
    pub(crate) callee: String,    // name of the callee symbol
    pub(crate) project_id: Arc<str>,
    pub(crate) caller_filepath: String, // to exclude self-calls from the MATCH filter
    pub(crate) allow_same_file: bool,
}

/// One inferred call edge (Swift extension resolution).
pub(crate) struct InferredCallRow {
    pub(crate) caller_id: String,
    pub(crate) callee: String,
    pub(crate) receiver_type: String,
    pub(crate) project_id: Arc<str>,
    pub(crate) caller_filepath: String,
    pub(crate) allow_same_file: bool,
}

pub(crate) struct PythonInferredCallRow {
    pub(crate) caller_id: String,
    pub(crate) callee: String,
    pub(crate) callee_filepath: String,
    pub(crate) project_id: Arc<str>,
    pub(crate) caller_filepath: String,
    pub(crate) allow_same_file: bool,
}

pub(crate) struct DbEdgeRow {
    pub(crate) src: String,
    pub(crate) tgt: String,
}

pub(crate) struct DbModelEdgeRow {
    pub(crate) src: String,
    pub(crate) model: String,
    pub(crate) project_id: Arc<str>,
}

pub(crate) struct ExternalApiNode {
    pub(crate) id: String,
    pub(crate) url: String,
    pub(crate) project_id: Arc<str>,
}

pub(crate) struct ExternalApiEdgeRow {
    pub(crate) src: String,
    pub(crate) tgt: String,
}

pub(crate) struct CloneGroupRow {
    id: String,
    project_id: String,
    size: usize,
    method: String,
    score_min: f64,
    score_max: f64,
    score_avg: f64,
}

pub(crate) struct CloneMemberRow {
    gid: String,
    sid: String,
}

pub(crate) struct CloneCanonRow {
    gid: String,
    sid: String,
}

pub(crate) struct FileCloneGroupRow {
    id: String,
    project_id: String,
    size: usize,
    method: String,
    score_min: f64,
    score_max: f64,
    score_avg: f64,
}

pub(crate) struct FileCloneMemberRow {
    gid: String,
    filepath: String,
    project_id: String,
}

pub(crate) struct FileCloneCanonRow {
    gid: String,
    filepath: String,
    project_id: String,
}

pub(crate) struct LaunchEdgeRow {
    pub(crate) src_filepath: String,
    pub(crate) tgt_filepath: String,
    pub(crate) project_id: String,
}

pub(crate) struct ImportSymbolRequest {
    pub(crate) src_id: String,
    pub(crate) src_filepath: String,
    pub(crate) module: String,
    pub(crate) items: Vec<String>,
}

pub(crate) struct ImportSymbolEdgeRow {
    pub(crate) src: String,
    pub(crate) tgt: String,
}

pub(crate) struct ImplicitImportSymbolEdgeRow {
    pub(crate) src: String,
    pub(crate) tgt: String,
}

pub(crate) struct ExportSymbolEdgeRow {
    pub(crate) src: String,
    pub(crate) tgt: String,
}

pub(crate) struct SwiftFileContext {
    pub(crate) file_id: String,
    pub(crate) filepath: String,
    pub(crate) symbol_spans: Vec<(usize, usize, String)>,
    pub(crate) extension_spans: Vec<(usize, usize, String)>,
    pub(crate) type_spans: Vec<(usize, usize, String)>,
    pub(crate) call_sites: Vec<tags::CallSite>,
    pub(crate) var_types: std::collections::HashMap<String, String>,
}

pub(crate) struct PythonFileContext {
    pub(crate) file_id: String,
    pub(crate) filepath: String,
    pub(crate) symbol_spans: Vec<(usize, usize, String)>,
    pub(crate) call_sites: Vec<tags::CallSite>,
    pub(crate) module_aliases: std::collections::HashMap<String, String>,
}

pub(crate) struct CloneCandidate {
    pub(crate) symbol_id: String,
    pub(crate) filepath: String,
    pub(crate) span_len: u32,
    pub(crate) token_set: HashSet<u64>,
    pub(crate) fingerprints: Vec<HashSet<u64>>,
    pub(crate) kgrams: HashSet<u64>,
}

impl SymbolCallRow {
    fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("caller_id".into(), Value::String(self.caller_id.clone()));
            m.insert("callee".into(), Value::String(self.callee.clone()));
            m.insert("pid".into(), Value::String(self.project_id.to_string()));
            m.insert("caller_fp".into(), Value::String(self.caller_filepath.clone()));
            m.insert("allow_same_file".into(), Value::Bool(self.allow_same_file));
            m
        })
    }
}

impl InferredCallRow {
    fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("caller_id".into(), Value::String(self.caller_id.clone()));
            m.insert("callee".into(), Value::String(self.callee.clone()));
            m.insert("recv".into(), Value::String(self.receiver_type.clone()));
            m.insert("pid".into(), Value::String(self.project_id.to_string()));
            m.insert("caller_fp".into(), Value::String(self.caller_filepath.clone()));
            m.insert("allow_same_file".into(), Value::Bool(self.allow_same_file));
            m
        })
    }
}

impl PythonInferredCallRow {
    fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("caller_id".into(), Value::String(self.caller_id.clone()));
            m.insert("callee".into(), Value::String(self.callee.clone()));
            m.insert("callee_fp".into(), Value::String(self.callee_filepath.clone()));
            m.insert("pid".into(), Value::String(self.project_id.to_string()));
            m.insert("caller_fp".into(), Value::String(self.caller_filepath.clone()));
            m.insert("allow_same_file".into(), Value::Bool(self.allow_same_file));
            m
        })
    }
}

impl DbEdgeRow {
    fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("src".into(), Value::String(self.src.clone()));
            m.insert("tgt".into(), Value::String(self.tgt.clone()));
            m
        })
    }
}

impl DbModelEdgeRow {
    fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("src".into(), Value::String(self.src.clone()));
            m.insert("model".into(), Value::String(self.model.clone()));
            m.insert("pid".into(), Value::String(self.project_id.to_string()));
            m
        })
    }
}

impl ExternalApiNode {
    fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("id".into(), Value::String(self.id.clone()));
            m.insert("url".into(), Value::String(self.url.clone()));
            m.insert("pid".into(), Value::String(self.project_id.to_string()));
            m
        })
    }
}

impl ExternalApiEdgeRow {
    fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("src".into(), Value::String(self.src.clone()));
            m.insert("tgt".into(), Value::String(self.tgt.clone()));
            m
        })
    }
}

impl CloneGroupRow {
    fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("id".into(), Value::String(self.id.clone()));
            m.insert("project_id".into(), Value::String(self.project_id.clone()));
            m.insert("size".into(), Value::Number(self.size.into()));
            m.insert("method".into(), Value::String(self.method.clone()));
            m.insert(
                "score_min".into(),
                Value::Number(serde_json::Number::from_f64(self.score_min).unwrap_or(0.into())),
            );
            m.insert(
                "score_max".into(),
                Value::Number(serde_json::Number::from_f64(self.score_max).unwrap_or(0.into())),
            );
            m.insert(
                "score_avg".into(),
                Value::Number(serde_json::Number::from_f64(self.score_avg).unwrap_or(0.into())),
            );
            m
        })
    }
}

impl CloneMemberRow {
    fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("gid".into(), Value::String(self.gid.clone()));
            m.insert("sid".into(), Value::String(self.sid.clone()));
            m
        })
    }
}

impl CloneCanonRow {
    fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("gid".into(), Value::String(self.gid.clone()));
            m.insert("sid".into(), Value::String(self.sid.clone()));
            m
        })
    }
}

impl FileCloneGroupRow {
    fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("id".into(), Value::String(self.id.clone()));
            m.insert("project_id".into(), Value::String(self.project_id.clone()));
            m.insert("size".into(), Value::Number(self.size.into()));
            m.insert("method".into(), Value::String(self.method.clone()));
            m.insert(
                "score_min".into(),
                Value::Number(serde_json::Number::from_f64(self.score_min).unwrap_or(0.into())),
            );
            m.insert(
                "score_max".into(),
                Value::Number(serde_json::Number::from_f64(self.score_max).unwrap_or(0.into())),
            );
            m.insert(
                "score_avg".into(),
                Value::Number(serde_json::Number::from_f64(self.score_avg).unwrap_or(0.into())),
            );
            m
        })
    }
}

impl FileCloneMemberRow {
    fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("gid".into(), Value::String(self.gid.clone()));
            m.insert("filepath".into(), Value::String(self.filepath.clone()));
            m.insert("project_id".into(), Value::String(self.project_id.clone()));
            m
        })
    }
}

impl FileCloneCanonRow {
    fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("gid".into(), Value::String(self.gid.clone()));
            m.insert("filepath".into(), Value::String(self.filepath.clone()));
            m.insert("project_id".into(), Value::String(self.project_id.clone()));
            m
        })
    }
}

impl ImportSymbolEdgeRow {
    fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("src".into(), Value::String(self.src.clone()));
            m.insert("tgt".into(), Value::String(self.tgt.clone()));
            m
        })
    }
}

impl ImplicitImportSymbolEdgeRow {
    fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("src".into(), Value::String(self.src.clone()));
            m.insert("tgt".into(), Value::String(self.tgt.clone()));
            m
        })
    }
}

impl ExportSymbolEdgeRow {
    fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("src".into(), Value::String(self.src.clone()));
            m.insert("tgt".into(), Value::String(self.tgt.clone()));
            m
        })
    }
}

impl LaunchEdgeRow {
    fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("src".into(), Value::String(self.src_filepath.clone()));
            m.insert("tgt".into(), Value::String(self.tgt_filepath.clone()));
            m.insert("pid".into(), Value::String(self.project_id.clone()));
            m
        })
    }
}

pub(crate) struct ImportNode {
    pub(crate) id: String,
    pub(crate) file_id: String,
    pub(crate) name: String,
    pub(crate) source: String,
    pub(crate) is_wildcard: bool,
    pub(crate) project_id: Arc<str>,
    pub(crate) filepath: String,
}

// Conversion to BoltType-compatible Value (we still use Value here to stay
// compatible with neo4rs `.param()` API, but we build it directly without
// serde round-trips through serde_json::json! macros in hot paths).

impl FileNode {
    fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("id".into(), Value::String(self.id.clone()));
            m.insert("name".into(), Value::String(self.name.clone()));
            m.insert("filepath".into(), Value::String(self.filepath.clone()));
            m.insert("project_id".into(), Value::String(self.project_id.to_string()));
            m
        })
    }
}

impl SymbolNode {
    fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("id".into(), Value::String(self.id.clone()));
            m.insert("name".into(), Value::String(self.name.clone()));
            m.insert("kind".into(), Value::String(self.kind.clone()));
            m.insert(
                "qualified_name".into(),
                self.qualified_name
                    .as_deref()
                    .map(|s| Value::String(s.into()))
                    .unwrap_or(Value::Null),
            );
            m.insert("filepath".into(), Value::String(self.filepath.clone()));
            m.insert("project_id".into(), Value::String(self.project_id.to_string()));
            m.insert("start_line".into(), Value::Number(self.start_line.into()));
            m.insert("end_line".into(), Value::Number(self.end_line.into()));
            m.insert(
                "start_byte".into(),
                Value::Number(serde_json::Number::from(self.start_byte)),
            );
            m.insert(
                "end_byte".into(),
                Value::Number(serde_json::Number::from(self.end_byte)),
            );
            m.insert(
                "signature".into(),
                self.signature
                    .as_deref()
                    .map(|s| Value::String(s.into()))
                    .unwrap_or(Value::Null),
            );
            m.insert(
                "visibility".into(),
                self.visibility
                    .as_deref()
                    .map(|s| Value::String(s.into()))
                    .unwrap_or(Value::Null),
            );
            m.insert("is_exported".into(), Value::Bool(self.is_exported));
            m.insert(
                "doc_comment".into(),
                self.doc_comment
                    .as_deref()
                    .map(|s| Value::String(s.into()))
                    .unwrap_or(Value::Null),
            );
            m
        })
    }
}

impl ImportNode {
    fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("id".into(), Value::String(self.id.clone()));
            m.insert("file_id".into(), Value::String(self.file_id.clone()));
            m.insert("name".into(), Value::String(self.name.clone()));
            m.insert("source".into(), Value::String(self.source.clone()));
            m.insert("is_wildcard".into(), Value::Bool(self.is_wildcard));
            m.insert("project_id".into(), Value::String(self.project_id.to_string()));
            m.insert("filepath".into(), Value::String(self.filepath.clone()));
            m
        })
    }
}

impl RelRow {
    fn to_value(&self) -> Value {
        Value::Object({
            let mut m = serde_json::Map::new();
            m.insert("p".into(), Value::String(self.parent.clone()));
            m.insert("c".into(), Value::String(self.child.clone()));
            m
        })
    }
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
        let _ = graph.run(Query::new(ddl.to_string())).await;
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
    let mut db_delegates_by_file: Vec<(String, String)> = Vec::new();
    let mut external_api_edges: Vec<ExternalApiEdgeRow> = Vec::new();
    let mut external_api_urls: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut seen_external_edges: std::collections::HashSet<(String, String)> = std::collections::HashSet::new();
    let mut import_symbol_edges: Vec<ImportSymbolEdgeRow> = Vec::new();
    let mut implicit_import_symbol_edges: Vec<ImplicitImportSymbolEdgeRow> = Vec::new();
    let mut export_symbol_edges: Vec<ExportSymbolEdgeRow> = Vec::new();
    let mut launch_requests: Vec<(String, String)> = Vec::new();
    let mut seen_export_symbol: std::collections::HashSet<(String, String)> = std::collections::HashSet::new();
    let mut import_symbol_requests: Vec<ImportSymbolRequest> = Vec::new();

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
            all_symbol_call_rows.extend(res.symbol_calls);
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
            if !res.db_delegates.is_empty() {
                db_sources.push(file_id.clone());
                for name in res.db_delegates {
                    db_delegates_by_file.push((file_id.clone(), name));
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

    let prep = prep_phase::prepare_graph_facts(
        &all_symbols,
        &all_files,
        &project_id,
        project_root.as_deref(),
        &launch_requests,
        &import_symbol_requests,
        &swift_extension_map,
        &swift_contexts,
        &python_contexts,
    );
    import_symbol_edges.extend(prep.import_symbol_edges);
    implicit_import_symbol_edges.extend(prep.implicit_import_symbol_edges);
    inferred_call_rows.extend(prep.inferred_call_rows);
    python_inferred_call_rows.extend(prep.python_inferred_call_rows);
    let launch_edges = prep.launch_edges;

    let mut manifest_abs: HashMap<String, String> = HashMap::new();
    for entry in &manifest {
        manifest_abs.insert(entry.rel_path.clone(), entry.abs_path.clone());
    }
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
            db_delegates_by_file,
            external_api_edges,
            external_api_urls,
            import_symbol_edges,
            implicit_import_symbol_edges,
            export_symbol_edges,
            launch_edges,
            manifest_abs,
        },
    )
    .await;

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

    let _ = graph
        .run(
            Query::new(
                "MERGE (r:IndexRun {id:$id}) \
                 SET r.project_id = $pid, \
                     r.status = 'done', \
                     r.finished_at = timestamp()"
                    .to_string(),
            )
            .param("id", run_id)
            .param("pid", config.project_id.to_string()),
        )
        .await;

    let indexed_paths: Vec<PathBuf> = manifest.into_iter().map(|m| PathBuf::from(m.abs_path)).collect();

    Ok(indexed_paths)
}
