mod clone_enrich;
mod pathing;
mod parse_phase;
mod swift;
mod tags;
mod writers;

use futures::{StreamExt, stream};
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
const NODE_BATCH_SIZE: usize = 5000;
/// Number of :CONTAINS / :IMPORTS relationships per UNWIND batch.
/// Smaller than nodes: each row acquires 2 read locks + 1 write scan.
const REL_BATCH_SIZE: usize = 1000;
/// Number of Import nodes per UNWIND batch
const IMPORT_BATCH_SIZE: usize = 2000;
/// Number of CallSiteRow items per CALLS write batch (each may unwind many callees)
const CALLS_BATCH_SIZE: usize = 200;

/// Concurrent writers for node phases.
/// Neo4j Community Edition serialises above 4 concurrent writers internally;
/// higher values just add connection + lock-queue overhead.
const NODE_CONCURRENCY: usize = 4;

/// Concurrent writers for relationship MERGE.
/// Relationship MERGE scans all edges from the parent node (O(degree)).
/// 2 is the sweet spot on Community Edition for this query shape.
const REL_CONCURRENCY: usize = 2;

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

fn external_api_id(project_id: &str, url: &str) -> String {
    pathing::external_api_id(project_id, url)
}

fn clean_import_name(name: &str) -> String {
    pathing::clean_import_name(name)
}

fn project_root_from_manifest(manifest: &[ManifestEntry]) -> Option<String> {
    pathing::project_root_from_manifest(manifest)
}

fn build_swift_module_map(project_root: &str, files_set: &HashSet<String>) -> HashMap<String, Vec<String>> {
    pathing::build_swift_module_map(project_root, files_set)
}

fn resolve_module_path(src_fp: &str, module: &str, files_set: &HashSet<String>) -> Option<String> {
    pathing::resolve_module_path(src_fp, module, files_set)
}

fn resolve_launch_path(src_fp: &str, raw: &str, project_root: &str, files_set: &HashSet<String>) -> Option<String> {
    pathing::resolve_launch_path(src_fp, raw, project_root, files_set)
}

fn extract_prisma_models(schema_text: &str) -> Vec<String> {
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
// Swift inference helpers
// ---------------------------------------------------------------------------

fn normalize_swift_type(raw: &str) -> Option<String> {
    swift::normalize_swift_type(raw)
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
    caller_id: String,
    callee: String,
    receiver_type: String,
    project_id: Arc<str>,
    caller_filepath: String,
    allow_same_file: bool,
}

pub(crate) struct PythonInferredCallRow {
    caller_id: String,
    callee: String,
    callee_filepath: String,
    project_id: Arc<str>,
    caller_filepath: String,
    allow_same_file: bool,
}

pub(crate) struct DbEdgeRow {
    src: String,
    tgt: String,
}

pub(crate) struct DbModelEdgeRow {
    src: String,
    model: String,
    project_id: Arc<str>,
}

pub(crate) struct ExternalApiNode {
    id: String,
    url: String,
    project_id: Arc<str>,
}

pub(crate) struct ExternalApiEdgeRow {
    src: String,
    tgt: String,
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
    src_filepath: String,
    tgt_filepath: String,
    project_id: String,
}

pub(crate) struct ImportSymbolRequest {
    pub(crate) src_id: String,
    pub(crate) src_filepath: String,
    pub(crate) module: String,
    pub(crate) items: Vec<String>,
}

pub(crate) struct ImportSymbolEdgeRow {
    src: String,
    tgt: String,
}

pub(crate) struct ImplicitImportSymbolEdgeRow {
    src: String,
    tgt: String,
}

pub(crate) struct ExportSymbolEdgeRow {
    src: String,
    tgt: String,
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

async fn write_file_nodes(graph: &Arc<Graph>, batch: &[FileNode]) {
    writers::write_file_nodes(graph, batch).await;
}

async fn write_symbol_nodes(graph: &Arc<Graph>, batch: &[SymbolNode], label: &str) {
    writers::write_symbol_nodes(graph, batch, label).await;
}

async fn write_import_nodes(graph: &Arc<Graph>, batch: &[ImportNode]) {
    writers::write_import_nodes(graph, batch).await;
}

async fn write_relationships(graph: &Arc<Graph>, batch: &[RelRow]) {
    writers::write_relationships(graph, batch).await;
}

async fn write_calls(graph: &Arc<Graph>, batch: &[SymbolCallRow]) {
    writers::write_calls(graph, batch).await;
}

async fn write_inferred_calls(graph: &Arc<Graph>, batch: &[InferredCallRow]) {
    writers::write_inferred_calls(graph, batch).await;
}

async fn write_python_inferred_calls(graph: &Arc<Graph>, batch: &[PythonInferredCallRow]) {
    writers::write_python_inferred_calls(graph, batch).await;
}

async fn write_db_edges(graph: &Arc<Graph>, batch: &[DbEdgeRow]) {
    writers::write_db_edges(graph, batch).await;
}

async fn write_db_model_edges(graph: &Arc<Graph>, batch: &[DbModelEdgeRow]) {
    writers::write_db_model_edges(graph, batch).await;
}

async fn write_external_api_nodes(graph: &Arc<Graph>, batch: &[ExternalApiNode]) {
    writers::write_external_api_nodes(graph, batch).await;
}

async fn write_external_api_edges(graph: &Arc<Graph>, batch: &[ExternalApiEdgeRow]) {
    writers::write_external_api_edges(graph, batch).await;
}

async fn write_import_symbol_edges(graph: &Arc<Graph>, batch: &[ImportSymbolEdgeRow]) {
    writers::write_import_symbol_edges(graph, batch).await;
}

async fn write_implicit_import_symbol_edges(graph: &Arc<Graph>, batch: &[ImplicitImportSymbolEdgeRow]) {
    writers::write_implicit_import_symbol_edges(graph, batch).await;
}

async fn write_export_symbol_edges(graph: &Arc<Graph>, batch: &[ExportSymbolEdgeRow]) {
    writers::write_export_symbol_edges(graph, batch).await;
}

async fn write_launch_edges(graph: &Arc<Graph>, batch: &[LaunchEdgeRow]) {
    writers::write_launch_edges(graph, batch).await;
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
    let mut seen_import_symbol: std::collections::HashSet<(String, String)> = std::collections::HashSet::new();
    let mut seen_implicit_import_symbol: std::collections::HashSet<(String, String)> = std::collections::HashSet::new();
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
    let project_root_str = project_root.as_deref().unwrap_or("");
    let mut launch_edges: Vec<LaunchEdgeRow> = Vec::new();
    if std::env::var("TS_PACK_LAUNCH_EDGES")
        .ok()
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
    {
        let mut seen_launch: std::collections::HashSet<(String, String)> = std::collections::HashSet::new();
        for (src_fp, raw) in &launch_requests {
            let Some(tgt_fp) = resolve_launch_path(src_fp, raw, project_root_str, &files_set) else {
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
        .as_deref()
        .map(|root| build_swift_module_map(root, &files_set))
        .unwrap_or_default();
    let mut swift_file_modules: HashMap<String, Vec<String>> = HashMap::new();
    for (module, module_files) in &swift_module_map {
        for fp in module_files {
            swift_file_modules.entry(fp.clone()).or_default().push(module.clone());
        }
    }
    let swift_implicit_imports = std::env::var("TS_PACK_SWIFT_IMPLICIT_IMPORTS")
        .ok()
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    for req in &import_symbol_requests {
        let target_fp = resolve_module_path(&req.src_filepath, &req.module, &files_set);
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
            let name = clean_import_name(item);
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

    if swift_implicit_imports {
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
                        if let Some(sym_map) = symbols_by_file.get(fp) {
                            for sym_id in sym_map.values() {
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

    if !swift_extension_map.is_empty() && !swift_contexts.is_empty() {
        let allow_same_file = std::env::var("TS_PACK_INCLUDE_INTRA_FILE_CALLS")
            .ok()
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let mut seen: std::collections::HashSet<(String, String, String)> = std::collections::HashSet::new();
        for ctx in &swift_contexts {
            for call in &ctx.call_sites {
                let Some(recv_raw) = &call.receiver else {
                    continue;
                };

                let recv = recv_raw.trim_end_matches(|c| c == '?' || c == '!');
                if recv.is_empty() {
                    continue;
                }

                let mut norm_ty = ctx.var_types.get(recv).and_then(|t| normalize_swift_type(t));

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
                        norm_ty = normalize_swift_type(recv);
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
                            project_id: Arc::clone(&project_id),
                            caller_filepath: ctx.filepath.clone(),
                            allow_same_file,
                        });
                    }
                }
            }
        }
    }

    let python_attr_calls = std::env::var("TS_PACK_PY_ATTR_CALLS")
        .ok()
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    if python_attr_calls && !python_contexts.is_empty() {
        let allow_same_file = std::env::var("TS_PACK_INCLUDE_INTRA_FILE_CALLS")
            .ok()
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let mut seen: std::collections::HashSet<(String, String, String)> = std::collections::HashSet::new();
        for ctx in &python_contexts {
            for call in &ctx.call_sites {
                let Some(recv) = &call.receiver else {
                    continue;
                };
                let Some(module) = ctx.module_aliases.get(recv) else {
                    continue;
                };
                let Some(module_fp) = resolve_module_path(&ctx.filepath, module, &files_set) else {
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
                        project_id: Arc::clone(&project_id),
                        caller_filepath: ctx.filepath.clone(),
                        allow_same_file,
                    });
                }
            }
        }
    }

    let mut manifest_abs: HashMap<String, String> = HashMap::new();
    for entry in &manifest {
        manifest_abs.insert(entry.rel_path.clone(), entry.abs_path.clone());
    }

    let schema_id = all_files
        .iter()
        .find(|f| f.filepath == "prisma/schema.prisma")
        .map(|f| f.id.clone());
    if let Some(schema_id) = schema_id {
        let _ = graph
            .run(
                Query::new("MATCH (:File {project_id: $pid})-[r:CALLS_DB]->() DELETE r".to_string())
                    .param("pid", project_id.to_string()),
            )
            .await;
        let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut db_edges: Vec<DbEdgeRow> = Vec::new();
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
                    let g = Arc::clone(&graph);
                    async move { write_db_edges(&g, chunk).await }
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
                let models = extract_prisma_models(&schema_text);
                for model in models {
                    if model.is_empty() {
                        continue;
                    }
                    model_map.insert(model.to_lowercase(), model.clone());
                    if let Some(first) = model.chars().next() {
                        let delegate = first.to_lowercase().collect::<String>() + &model[1..];
                        model_map.insert(delegate.to_lowercase(), model.clone());
                    }
                }

                let mut db_model_edges: Vec<DbModelEdgeRow> = Vec::new();
                for (file_id, delegate) in &db_delegates_by_file {
                    if let Some(model) = model_map.get(&delegate.to_lowercase()) {
                        db_model_edges.push(DbModelEdgeRow {
                            src: file_id.clone(),
                            model: model.clone(),
                            project_id: Arc::clone(&project_id),
                        });
                    }
                }

                let _ = graph
                    .run(
                        Query::new("MATCH (m:Model {project_id: $pid}) DETACH DELETE m".to_string())
                            .param("pid", project_id.to_string()),
                    )
                    .await;
                if !db_model_edges.is_empty() {
                    let t_dbm = Instant::now();
                    let dbm_count = db_model_edges.len();
                    stream::iter(db_model_edges.chunks(CALLS_BATCH_SIZE))
                        .for_each_concurrent(REL_CONCURRENCY, |chunk| {
                            let g = Arc::clone(&graph);
                            async move { write_db_model_edges(&g, chunk).await }
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

    let _ = graph
        .run(
            Query::new("MATCH (:File {project_id: $pid})-[r:CALLS_API_EXTERNAL]->() DELETE r".to_string())
                .param("pid", project_id.to_string()),
        )
        .await;
    let _ = graph
        .run(
            Query::new("MATCH (e:ExternalAPI {project_id: $pid}) DETACH DELETE e".to_string())
                .param("pid", project_id.to_string()),
        )
        .await;
    if !external_api_urls.is_empty() && !external_api_edges.is_empty() {
        let t_ext = Instant::now();
        let mut external_nodes: Vec<ExternalApiNode> = Vec::new();
        for url in &external_api_urls {
            external_nodes.push(ExternalApiNode {
                id: external_api_id(&project_id, url),
                url: url.clone(),
                project_id: Arc::clone(&project_id),
            });
        }
        stream::iter(external_nodes.chunks(NODE_BATCH_SIZE))
            .for_each_concurrent(NODE_CONCURRENCY, |chunk| {
                let g = Arc::clone(&graph);
                async move { write_external_api_nodes(&g, chunk).await }
            })
            .await;
        let ext_count = external_api_edges.len();
        stream::iter(external_api_edges.chunks(CALLS_BATCH_SIZE))
            .for_each_concurrent(REL_CONCURRENCY, |chunk| {
                let g = Arc::clone(&graph);
                async move { write_external_api_edges(&g, chunk).await }
            })
            .await;
        eprintln!(
            "[ts-pack-index] CALLS_API_EXTERNAL writes done in {:.2}s (rows={})",
            t_ext.elapsed().as_secs_f64(),
            ext_count,
        );
    }

    let _ = graph
        .run(
            Query::new("MATCH (:File {project_id: $pid})-[r:IMPORTS_SYMBOL]->() DELETE r".to_string())
                .param("pid", project_id.to_string()),
        )
        .await;
    let _ = graph
        .run(
            Query::new("MATCH (:File {project_id: $pid})-[r:IMPLICIT_IMPORTS_SYMBOL]->() DELETE r".to_string())
                .param("pid", project_id.to_string()),
        )
        .await;
    let _ = graph
        .run(
            Query::new("MATCH (:File {project_id: $pid})-[r:EXPORTS_SYMBOL]->() DELETE r".to_string())
                .param("pid", project_id.to_string()),
        )
        .await;

    // --- Phase 2: Write file nodes ----------------------------------------
    let t_nodes = Instant::now();

    stream::iter(all_files.chunks(NODE_BATCH_SIZE))
        .for_each_concurrent(NODE_CONCURRENCY, |chunk| {
            let g = Arc::clone(&graph);
            async move { write_file_nodes(&g, chunk).await }
        })
        .await;

    // Write symbol nodes per label group
    let symbol_labels: Vec<(&'static str, Vec<SymbolNode>)> = all_symbols.into_iter().collect();
    for (label, nodes) in &symbol_labels {
        stream::iter(nodes.chunks(NODE_BATCH_SIZE))
            .for_each_concurrent(NODE_CONCURRENCY, |chunk| {
                let g = Arc::clone(&graph);
                async move { write_symbol_nodes(&g, chunk, label).await }
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
            .for_each_concurrent(REL_CONCURRENCY, |chunk| {
                let g = Arc::clone(&graph);
                async move { write_import_symbol_edges(&g, chunk).await }
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
            .for_each_concurrent(REL_CONCURRENCY, |chunk| {
                let g = Arc::clone(&graph);
                async move { write_implicit_import_symbol_edges(&g, chunk).await }
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
            .for_each_concurrent(REL_CONCURRENCY, |chunk| {
                let g = Arc::clone(&graph);
                async move { write_export_symbol_edges(&g, chunk).await }
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
                let g = Arc::clone(&graph);
                async move { write_launch_edges(&g, chunk).await }
            })
            .await;
        eprintln!(
            "[ts-pack-index] LAUNCHES writes done in {:.2}s (rows={})",
            t_launch.elapsed().as_secs_f64(),
            launch_count,
        );
    }

    // --- Phase 3: Write import nodes -------------------------------------
    let t_imports = Instant::now();
    let import_count = all_imports.len();

    stream::iter(all_imports.chunks(IMPORT_BATCH_SIZE))
        .for_each_concurrent(NODE_CONCURRENCY, |chunk| {
            let g = Arc::clone(&graph);
            async move { write_import_nodes(&g, chunk).await }
        })
        .await;

    eprintln!(
        "[ts-pack-index] Import writes done in {:.2}s (count={})",
        t_imports.elapsed().as_secs_f64(),
        import_count,
    );

    // --- Phase 4: Write CONTAINS relationships ---------------------------
    // Combine structural and import-edge relations into one flush.
    all_rels.extend(all_import_rels);
    let rel_count = all_rels.len();

    let t_rels = Instant::now();

    // Relationship MERGE: lowest concurrency to minimise lock contention.
    stream::iter(all_rels.chunks(REL_BATCH_SIZE))
        .for_each_concurrent(REL_CONCURRENCY, |chunk| {
            let g = Arc::clone(&graph);
            async move { write_relationships(&g, chunk).await }
        })
        .await;

    eprintln!(
        "[ts-pack-index] Relationship writes done in {:.2}s (count={})",
        t_rels.elapsed().as_secs_f64(),
        rel_count,
    );

    // --- Phase 5: Write CALLS relationships --------------------------------
    // Resolve symbol-level call edges (Symbol→Symbol, File→Symbol fallback).
    let t_calls = Instant::now();
    let calls_row_count = all_symbol_call_rows.len();

    stream::iter(all_symbol_call_rows.chunks(CALLS_BATCH_SIZE))
        .for_each_concurrent(REL_CONCURRENCY, |chunk| {
            let g = Arc::clone(&graph);
            async move { write_calls(&g, chunk).await }
        })
        .await;

    // --- Phase 6: Clone grouping (Rust) ------------------------------------
    let clone_enabled = std::env::var("LM_PROXY_CLONE_ENRICH")
        .ok()
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true);
    if clone_enabled && !clone_candidates.is_empty() {
        let cfg = clone_enrich::CloneConfig {
            min_overlap: WINNOW_MIN_OVERLAP,
            token_sim_threshold: WINNOW_TOKEN_SIM_THRESHOLD,
            kgram_sim_threshold: WINNOW_KGRAM_SIM_THRESHOLD,
            min_score: WINNOW_MIN_SCORE,
            bucket_limit: WINNOW_BUCKET_LIMIT,
            fallback_hashes: WINNOW_FALLBACK_HASHES,
            force_all_hashes_max_fps: WINNOW_FORCE_ALL_HASHES_MAX_FPS,
        };
        clone_enrich::write_clone_enrichment(
            &graph,
            project_id.as_ref(),
            &clone_candidates,
            REL_BATCH_SIZE,
            REL_CONCURRENCY,
            &cfg,
        )
        .await;
    }

    eprintln!(
        "[ts-pack-index] CALLS writes done in {:.2}s (rows={})",
        t_calls.elapsed().as_secs_f64(),
        calls_row_count,
    );

    if !inferred_call_rows.is_empty() || !python_inferred_call_rows.is_empty() {
        let t_inf = Instant::now();
        let swift_count = inferred_call_rows.len();
        let py_count = python_inferred_call_rows.len();

        if !inferred_call_rows.is_empty() {
            stream::iter(inferred_call_rows.chunks(CALLS_BATCH_SIZE))
                .for_each_concurrent(REL_CONCURRENCY, |chunk| {
                    let g = Arc::clone(&graph);
                    async move { write_inferred_calls(&g, chunk).await }
                })
                .await;
        }

        if !python_inferred_call_rows.is_empty() {
            stream::iter(python_inferred_call_rows.chunks(CALLS_BATCH_SIZE))
                .for_each_concurrent(REL_CONCURRENCY, |chunk| {
                    let g = Arc::clone(&graph);
                    async move { write_python_inferred_calls(&g, chunk).await }
                })
                .await;
        }

        eprintln!(
            "[ts-pack-index] CALLS_INFERRED writes done in {:.2}s (rows={})",
            t_inf.elapsed().as_secs_f64(),
            swift_count + py_count,
        );
    }

    // --- Summary ----------------------------------------------------------
    let total_elapsed = t0.elapsed();
    eprintln!(
        "[ts-pack-index] Done — {total_files} files | \
         parse={:.2}s nodes={:.2}s imports={:.2}s rels={:.2}s calls={:.2}s total={:.2}s",
        parse_elapsed.as_secs_f64(),
        node_elapsed.as_secs_f64(),
        t_imports.elapsed().as_secs_f64(),
        t_rels.elapsed().as_secs_f64(),
        t_calls.elapsed().as_secs_f64(),
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
