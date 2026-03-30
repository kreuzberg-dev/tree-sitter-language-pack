mod tags;

use futures::{StreamExt, stream};
use neo4rs::{BoltType, Graph, Query};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
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
const MAX_FILE_BYTES: usize = 1_000_000;

// ---------------------------------------------------------------------------
// Typed payload structs (avoids serde_json::json! round-trips in hot loops)
// ---------------------------------------------------------------------------

struct FileNode {
    id: String,
    name: String,
    filepath: String,
    project_id: Arc<str>,
}

struct SymbolNode {
    id: String,
    name: String,
    kind: String,
    qualified_name: Option<String>,
    filepath: String,
    project_id: Arc<str>,
    start_line: u32,
    end_line: u32,
    start_byte: usize,
    end_byte: usize,
    signature: Option<String>,
    visibility: Option<String>,
    is_exported: bool,
    doc_comment: Option<String>,
}

struct RelRow {
    parent: String,
    child: String,
}

/// One resolved call edge: caller is a Symbol (or File as fallback) → callee symbol name.
struct SymbolCallRow {
    caller_id: String, // id of the calling Symbol node (or File if at file scope)
    callee: String,    // name of the callee symbol
    project_id: Arc<str>,
    caller_filepath: String, // to exclude self-calls from the MATCH filter
    allow_same_file: bool,
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

struct ImportNode {
    id: String,
    file_id: String,
    name: String,
    source: String,
    is_wildcard: bool,
    project_id: Arc<str>,
    filepath: String,
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
// JSON → BoltType adapter (used only at the neo4rs boundary)
// ---------------------------------------------------------------------------

fn json_to_bolt(v: Value) -> BoltType {
    match v {
        Value::String(s) => BoltType::from(s),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                BoltType::from(i)
            } else if let Some(f) = n.as_f64() {
                BoltType::from(f)
            } else {
                BoltType::from(0i64)
            }
        }
        Value::Bool(b) => BoltType::from(b),
        Value::Null => BoltType::Null(neo4rs::BoltNull),
        Value::Array(arr) => BoltType::from(arr.into_iter().map(json_to_bolt).collect::<Vec<_>>()),
        Value::Object(map) => {
            let mut bolt_map = HashMap::new();
            for (k, val) in map {
                bolt_map.insert(k, json_to_bolt(val));
            }
            BoltType::from(bolt_map)
        }
    }
}

/// Convert a slice of rows to a BoltType list without an intermediate Vec clone.
fn rows_to_bolt<T, F: Fn(&T) -> Value>(rows: &[T], f: F) -> BoltType {
    BoltType::from(rows.iter().map(|r| json_to_bolt(f(r))).collect::<Vec<_>>())
}

// ---------------------------------------------------------------------------
// Write helpers — one per entity kind
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Write helpers — ON CREATE / ON MATCH split on every MERGE.
//
// WHY: On a non-empty graph every unconditional SET dirties the page even
// when nothing changed. ON CREATE SET runs once; ON MATCH SET updates only
// what can legitimately change across re-index runs (name, line numbers).
// This halves write-amplification on warm re-indexes.
// ---------------------------------------------------------------------------

async fn write_file_nodes(graph: &Arc<Graph>, batch: &[FileNode]) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MERGE (n:Node {id: item.id}) \
         ON CREATE SET n:File, \
                       n.name       = item.name, \
                       n.filepath   = item.filepath, \
                       n.project_id = item.project_id \
         ON MATCH SET  n.name       = item.name"
            .to_string(),
    )
    .param("batch", bolt);
    let _ = graph.run(q).await;
}

async fn write_symbol_nodes(graph: &Arc<Graph>, batch: &[SymbolNode], label: &str) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    // ON CREATE: write all properties and add the specific label.
    // ON MATCH:  only update line numbers (the only thing that moves on edit).
    //            The extra label is NOT re-applied on match — Neo4j ignores
    //            SET n:Label when the label is already present, but skipping
    //            it here avoids the label-index re-evaluation overhead.
    let cypher = format!(
        "UNWIND $batch AS item \
         MERGE (n:Node {{id: item.id}}) \
         ON CREATE SET n:{label}, \
                       n.name        = item.name, \
                       n.kind        = item.kind, \
                       n.qualified_name = item.qualified_name, \
                       n.project_id  = item.project_id, \
                       n.filepath    = item.filepath, \
                       n.start_line  = item.start_line, \
                       n.end_line    = item.end_line, \
                       n.start_byte  = item.start_byte, \
                       n.end_byte    = item.end_byte, \
                       n.signature   = item.signature, \
                       n.visibility  = item.visibility, \
                       n.is_exported = item.is_exported, \
                       n.doc_comment = item.doc_comment \
         ON MATCH SET  n.start_line  = item.start_line, \
                       n.end_line    = item.end_line, \
                       n.qualified_name = item.qualified_name, \
                       n.signature   = item.signature, \
                       n.visibility  = item.visibility, \
                       n.is_exported = item.is_exported, \
                       n.doc_comment = item.doc_comment \
         FOREACH (_ IN CASE WHEN item.kind = 'Method' THEN [1] ELSE [] END | SET n:Method)"
    );
    let q = Query::new(cypher).param("batch", bolt);
    let _ = graph.run(q).await;
}

async fn write_import_nodes(graph: &Arc<Graph>, batch: &[ImportNode]) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MERGE (n:Node {id: item.id}) \
         ON CREATE SET n:Import, \
                       n.name        = item.name, \
                       n.source      = item.source, \
                       n.is_wildcard = item.is_wildcard, \
                       n.project_id  = item.project_id, \
                       n.filepath    = item.filepath"
            .to_string(),
    )
    .param("batch", bolt);
    let _ = graph.run(q).await;
}

async fn write_relationships(graph: &Arc<Graph>, batch: &[RelRow]) {
    // Relationship MERGE is the most lock-sensitive query:
    // each row acquires write locks on p, c, and scans p's outgoing edges.
    // Keep batch size ≤ 1000 and concurrency ≤ 2 (see REL_CONCURRENCY).
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (p:Node {id: item.p}) \
         MATCH (c:Node {id: item.c}) \
         MERGE (p)-[:CONTAINS]->(c)"
            .to_string(),
    )
    .param("batch", bolt);
    let _ = graph.run(q).await;
}

async fn write_calls(graph: &Arc<Graph>, batch: &[SymbolCallRow]) {
    // For each (caller, callee_name) pair, MERGE a :CALLS edge from the caller
    // Symbol (or File) node to any matching Symbol in the same project whose
    // filepath differs from the caller's file (no self-file edges).
    // Multiple symbols with the same name all receive the edge — intentional.
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (caller:Node {id: item.caller_id}) \
         MATCH (callee:Node {project_id: item.pid, name: item.callee}) \
         WHERE (callee:Function OR callee:Class OR callee:Struct OR callee:Method) \
           AND (item.allow_same_file = true OR callee.filepath <> item.caller_fp) \
         MERGE (caller)-[:CALLS]->(callee)"
            .to_string(),
    )
    .param("batch", bolt);
    let _ = graph.run(q).await;
}

// ---------------------------------------------------------------------------
// Per-file parse output
// ---------------------------------------------------------------------------

struct FileResult {
    file_node: FileNode,
    symbols: HashMap<&'static str, Vec<SymbolNode>>,
    relations: Vec<RelRow>,
    imports: Vec<ImportNode>,
    import_rels: Vec<RelRow>,
    symbol_calls: Vec<SymbolCallRow>, // attributed call edges (Symbol→Symbol or File→Symbol)
}

// ---------------------------------------------------------------------------
// Symbol-tree walker (recursive, avoids re-allocating label strings)
// ---------------------------------------------------------------------------

fn walk_item(
    item: &ts_pack::StructureItem,
    parent_id: &str,
    filepath: &str,
    project_id: Arc<str>,
    exported_names: &std::collections::HashSet<String>,
    symbols: &mut HashMap<&'static str, Vec<SymbolNode>>,
    relations: &mut Vec<RelRow>,
) {
    let label: &'static str = match item.kind {
        ts_pack::StructureKind::Class => "Class",
        ts_pack::StructureKind::Function | ts_pack::StructureKind::Method => "Function",
        ts_pack::StructureKind::Interface => "Interface",
        ts_pack::StructureKind::Protocol => "Protocol",
        ts_pack::StructureKind::Trait => "Trait",
        ts_pack::StructureKind::Impl => "Impl",
        ts_pack::StructureKind::Struct => "Struct",
        ts_pack::StructureKind::Enum => "Enum",
        ts_pack::StructureKind::EnumCase => "EnumCase",
        ts_pack::StructureKind::Extension => "Extension",
        ts_pack::StructureKind::TypeAlias => "TypeAlias",
        ts_pack::StructureKind::AssociatedType => "AssociatedType",
        ts_pack::StructureKind::Module | ts_pack::StructureKind::Namespace => "Namespace",
        ts_pack::StructureKind::Section => "Section",
        _ => "Symbol",
    };

    let name = item.name.as_deref().unwrap_or("unnamed");
    // ID encodes project, kind, file, and name. Position (start_line/start_byte)
    // is intentionally excluded so that MERGE correctly matches an existing symbol
    // node after the file is edited and line numbers shift — avoiding ghost duplicates.
    let node_id = format!("{}:{}:{}:{}", project_id, label.to_ascii_lowercase(), filepath, name,);

    // is_exported: true if visibility is public/pub, or if the name appears in result.exports
    let is_exported = item
        .visibility
        .as_deref()
        .map(|v| v == "public" || v == "pub" || v.starts_with("pub("))
        .unwrap_or(false)
        || exported_names.contains(name);

    symbols.entry(label).or_default().push(SymbolNode {
        id: node_id.clone(),
        name: name.to_string(),
        kind: format!("{:?}", item.kind),
        qualified_name: item.qualified_name.clone(),
        filepath: filepath.to_string(),
        project_id: Arc::clone(&project_id),
        start_line: (item.span.start_line + 1) as u32,
        end_line: (item.span.end_line + 1) as u32,
        start_byte: item.span.start_byte,
        end_byte: item.span.end_byte,
        signature: item.signature.clone(),
        visibility: item.visibility.clone(),
        is_exported,
        doc_comment: item.doc_comment.clone(),
    });
    relations.push(RelRow {
        parent: parent_id.to_string(),
        child: node_id.clone(),
    });

    for child in &item.children {
        walk_item(
            child,
            &node_id,
            filepath,
            Arc::clone(&project_id),
            exported_names,
            symbols,
            relations,
        );
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

        let pid = Arc::clone(&project_id);
        let parse_entry = |entry: &ManifestEntry| {
            // Language detection
            let lang_name = ts_pack::detect_language_from_extension(&entry.ext)?;
            if !ts_pack::has_language(lang_name) {
                if let Err(err) = ts_pack::download(&[lang_name]) {
                    eprintln!("[ts-pack-index] download failed: {lang} ({err})", lang = lang_name);
                    return None;
                }
            }

            // Read source — skip oversized files
            let source = std::fs::read_to_string(&entry.abs_path).ok()?;
            if source.len() > MAX_FILE_BYTES {
                return None;
            }

            let proc_config = ts_pack::ProcessConfig::new(lang_name).all();
            let result = ts_pack::process(&source, &proc_config).ok()?;

            let rel_path = &entry.rel_path;
            let file_name = PathBuf::from(&entry.abs_path)
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            let file_id = format!("{}:file:{}", pid, rel_path);

            let file_node = FileNode {
                id: file_id.clone(),
                name: file_name,
                filepath: rel_path.clone(),
                project_id: Arc::clone(&pid),
            };

            let mut symbols: HashMap<&'static str, Vec<SymbolNode>> = HashMap::new();
            let mut relations: Vec<RelRow> = Vec::new();
            let mut imports: Vec<ImportNode> = Vec::new();
            let mut import_rels: Vec<RelRow> = Vec::new();

            // Build exported-name set from structural result + tags visibility
            let mut exported_names: std::collections::HashSet<String> =
                result.exports.iter().map(|e| e.name.clone()).collect();
            let tags_result = ts_pack::parse_string(lang_name, source.as_bytes())
                .ok()
                .and_then(|tree| tags::run_tags(lang_name, &tree, source.as_bytes()));

            // --- Consume tags result: split into exported names + call sites ---
            let (tag_exported, raw_call_sites) = match tags_result {
                Some(tr) => (tr.exported_names, tr.call_sites),
                None => (std::collections::HashSet::new(), Vec::new()),
            };
            exported_names.extend(tag_exported);

            // Walk structural tree (populates `symbols` with start/end bytes)
            for item in &result.structure {
                walk_item(
                    item,
                    &file_id,
                    rel_path,
                    Arc::clone(&pid),
                    &exported_names,
                    &mut symbols,
                    &mut relations,
                );
            }

            // --- Span correlation: attribute each call site to its enclosing symbol ---
            // Build a flat list of (start_byte, end_byte, symbol_id) from all SymbolNodes.
            // The innermost (smallest) enclosing span wins — handles nested functions.
            let symbol_spans: Vec<(usize, usize, String)> = symbols
                .values()
                .flat_map(|v| v.iter())
                .map(|s| (s.start_byte, s.end_byte, s.id.clone()))
                .collect();

            let mut seen_calls: std::collections::HashSet<(String, String)> = std::collections::HashSet::new();

            let allow_same_file = std::env::var("TS_PACK_INCLUDE_INTRA_FILE_CALLS")
                .ok()
                .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                .unwrap_or(false);

            let symbol_calls: Vec<SymbolCallRow> = raw_call_sites
                .into_iter()
                .filter_map(|cs| {
                    // Find the innermost enclosing symbol (smallest span containing cs.start_byte)
                    let caller_id = symbol_spans
                        .iter()
                        .filter(|(sb, eb, _)| *sb <= cs.start_byte && cs.start_byte < *eb)
                        .min_by_key(|(sb, eb, _)| eb - sb)
                        .map(|(_, _, id)| id.clone())
                        .unwrap_or_else(|| file_id.clone()); // fallback: attribute to file

                    // Deduplicate (caller_id, callee) pairs within this file
                    if seen_calls.insert((caller_id.clone(), cs.callee.clone())) {
                        Some(SymbolCallRow {
                            caller_id,
                            callee: cs.callee,
                            project_id: Arc::clone(&pid),
                            caller_filepath: rel_path.clone(),
                            allow_same_file,
                        })
                    } else {
                        None
                    }
                })
                .collect();

            // Collect imports
            for imp in &result.imports {
                let import_id = format!("{}:import:{}:{}", pid, rel_path, imp.source);
                imports.push(ImportNode {
                    id: import_id.clone(),
                    file_id: file_id.clone(),
                    name: imp.source.clone(),
                    source: imp.source.clone(),
                    is_wildcard: imp.is_wildcard,
                    project_id: Arc::clone(&pid),
                    filepath: rel_path.clone(),
                });
                import_rels.push(RelRow {
                    parent: file_id.clone(),
                    child: import_id,
                });
            }

            Some(FileResult {
                file_node,
                symbols,
                relations,
                imports,
                import_rels,
                symbol_calls,
            })
        };

        let batch_results: Vec<FileResult> = if std::env::var("TS_PACK_SERIAL_PARSE").is_ok() {
            batch.iter().filter_map(parse_entry).collect()
        } else {
            batch.par_iter().filter_map(parse_entry).collect()
        };

        // Merge batch results into global reservoirs
        for res in batch_results {
            all_symbol_call_rows.extend(res.symbol_calls);
            all_files.push(res.file_node);
            for (label, syms) in res.symbols {
                all_symbols.entry(label).or_default().extend(syms);
            }
            all_rels.extend(res.relations);
            all_imports.extend(res.imports);
            all_import_rels.extend(res.import_rels);
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

    eprintln!(
        "[ts-pack-index] CALLS writes done in {:.2}s (rows={})",
        t_calls.elapsed().as_secs_f64(),
        calls_row_count,
    );

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

    let indexed_paths: Vec<PathBuf> = manifest.into_iter().map(|m| PathBuf::from(m.abs_path)).collect();

    Ok(indexed_paths)
}
