mod tags;

use futures::{StreamExt, stream};
use neo4rs::{BoltType, Graph, Query};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
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

fn external_api_id(project_id: &str, url: &str) -> String {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    url.hash(&mut hasher);
    format!("{}:external:{}", project_id, format!("{:x}", hasher.finish()))
}

fn join_url(base: &str, path: &str) -> Option<String> {
    if base.is_empty() || path.is_empty() {
        return None;
    }
    if path.starts_with("http://") || path.starts_with("https://") {
        return Some(path.to_string());
    }
    if !(base.starts_with("http://") || base.starts_with("https://") || base.starts_with("env://")) {
        return None;
    }
    let mut base = base.to_string();
    let mut path = path.to_string();
    if base.ends_with('/') && path.starts_with('/') {
        path.remove(0);
    } else if !base.ends_with('/') && !path.starts_with('/') {
        base.push('/');
    }
    Some(format!("{base}{path}"))
}

fn clean_import_name(name: &str) -> String {
    let mut out = name.trim().to_string();
    for prefix in ["type ", "typeof "] {
        if out.starts_with(prefix) {
            out = out[prefix.len()..].trim().to_string();
        }
    }
    if let Some((before, _)) = out.split_once(" as ") {
        out = before.trim().to_string();
    }
    out
}

fn resolve_module_path(src_fp: &str, module: &str, files_set: &HashSet<String>) -> Option<String> {
    let module = module.trim();
    if module.is_empty() {
        return None;
    }

    let base = if module.starts_with("./") || module.starts_with("../") {
        let mut base = std::path::PathBuf::from(src_fp);
        base.pop();
        base.push(module);
        let mut parts: Vec<String> = Vec::new();
        for comp in base.components() {
            use std::path::Component;
            match comp {
                Component::ParentDir => {
                    parts.pop();
                }
                Component::CurDir => {}
                Component::Normal(val) => parts.push(val.to_string_lossy().to_string()),
                _ => {}
            }
        }
        parts.join("/")
    } else if module.starts_with("@/") || module.starts_with("~/") {
        module[2..].to_string()
    } else if module.starts_with("src/") {
        module.to_string()
    } else if src_fp.ends_with(".py") {
        let mut mod_str = module.to_string();
        let mut dot_count = 0usize;
        while mod_str.starts_with('.') {
            dot_count += 1;
            mod_str.remove(0);
        }

        let base = if dot_count > 0 {
            let mut base = std::path::PathBuf::from(src_fp);
            base.pop();
            for _ in 1..dot_count {
                base.pop();
            }
            if !mod_str.is_empty() {
                base.push(mod_str.replace('.', "/"));
            }
            base
        } else {
            std::path::PathBuf::from(mod_str.replace('.', "/"))
        };

        let base_str = base
            .to_string_lossy()
            .replace('\\', "/")
            .trim_end_matches('/')
            .to_string();
        if base_str.is_empty() {
            return None;
        }
        for suf in [".py", "/__init__.py"] {
            let candidate = format!("{base_str}{suf}");
            if files_set.contains(&candidate) {
                return Some(candidate);
            }
        }
        return None;
    } else {
        return None;
    };

    for suf in [
        "",
        ".ts",
        ".tsx",
        ".js",
        ".jsx",
        "/index.ts",
        "/index.tsx",
        "/index.js",
        "/index.jsx",
    ] {
        let candidate = format!("{base}{suf}");
        if files_set.contains(&candidate) {
            return Some(candidate);
        }
    }

    None
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
    let mut s = raw.trim().trim_end_matches('?').trim_end_matches('!').to_string();
    if let Some(idx) = s.find('<') {
        s.truncate(idx);
    }
    if s.is_empty() { None } else { Some(s) }
}

fn parse_swift_var_types(source: &str) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    let extract_type_from_rhs = |rhs: &str| -> Option<String> {
        let mut rhs = rhs.trim();
        if rhs.is_empty() {
            return None;
        }
        if let Some(idx) = rhs.find(" as? ") {
            rhs = rhs[idx + 5..].trim();
        } else if let Some(idx) = rhs.find(" as ") {
            rhs = rhs[idx + 4..].trim();
        }
        let mut ty = String::new();
        for ch in rhs.chars() {
            if ch.is_alphanumeric() || ch == '_' || ch == '.' || ch == '<' || ch == '>' || ch == '?' || ch == '!' {
                ty.push(ch);
            } else {
                break;
            }
        }
        if let Some(tn) = normalize_swift_type(&ty) {
            if let Some((head, _)) = tn.split_once('.') {
                return normalize_swift_type(head);
            }
            return Some(tn);
        }
        None
    };

    let extract_receiver_from_chain = |rhs: &str| -> Option<String> {
        let mut rhs = rhs.trim();
        if rhs.is_empty() {
            return None;
        }
        if let Some(idx) = rhs.find(" as? ") {
            rhs = rhs[..idx].trim();
        } else if let Some(idx) = rhs.find(" as ") {
            rhs = rhs[..idx].trim();
        }
        let mut name = String::new();
        for ch in rhs.chars() {
            if ch.is_alphanumeric() || ch == '_' {
                name.push(ch);
            } else {
                break;
            }
        }
        if name.is_empty() { None } else { Some(name) }
    };

    for line in source.lines() {
        let trimmed = line.trim();
        let (kw, rest) = if let Some(r) = trimmed.strip_prefix("let ") {
            ("let", r)
        } else if let Some(r) = trimmed.strip_prefix("var ") {
            ("var", r)
        } else if let Some(r) = trimmed.strip_prefix("if let ") {
            ("if let", r)
        } else if let Some(r) = trimmed.strip_prefix("guard let ") {
            ("guard let", r)
        } else {
            // Try reassignment: name = Type(...)
            if trimmed.contains('=')
                && !trimmed.contains("==")
                && !trimmed.contains("!=")
                && !trimmed.contains(">=")
                && !trimmed.contains("<=")
            {
                if let Some(eq_idx) = trimmed.find('=') {
                    let lhs = trimmed[..eq_idx].trim();
                    let rhs = trimmed[eq_idx + 1..].trim();
                    let mut name = String::new();
                    for ch in lhs.chars().rev() {
                        if ch.is_alphanumeric() || ch == '_' {
                            name.push(ch);
                        } else if !name.is_empty() {
                            break;
                        }
                    }
                    let name = name.chars().rev().collect::<String>();
                    if !name.is_empty() {
                        if let Some(tn) = extract_type_from_rhs(rhs) {
                            map.insert(name, tn);
                        } else if rhs.contains('.') {
                            if let Some(recv) = extract_receiver_from_chain(rhs) {
                                if let Some(tn) = map.get(&recv).cloned() {
                                    map.insert(name, tn);
                                }
                            }
                        }
                    }
                }
            }
            continue;
        };
        let rest = rest.trim();
        if rest.is_empty() {
            continue;
        }

        let mut name = String::new();
        for ch in rest.chars() {
            if ch.is_alphanumeric() || ch == '_' {
                name.push(ch);
            } else {
                break;
            }
        }
        if name.is_empty() {
            continue;
        }

        if let Some(idx) = rest.find(':') {
            let type_part = rest[idx + 1..].trim();
            let mut ty = String::new();
            for ch in type_part.chars() {
                if ch.is_alphanumeric() || ch == '_' || ch == '.' || ch == '<' || ch == '>' || ch == '?' || ch == '!' {
                    ty.push(ch);
                } else {
                    break;
                }
            }
            if let Some(tn) = normalize_swift_type(&ty) {
                map.insert(name, tn);
            }
            continue;
        }

        if let Some(eq_idx) = rest.find('=') {
            let rhs = rest[eq_idx + 1..].trim();
            if let Some(tn) = extract_type_from_rhs(rhs) {
                map.insert(name, tn);
            } else if rhs.contains('.') {
                if let Some(recv) = extract_receiver_from_chain(rhs) {
                    if let Some(tn) = map.get(&recv).cloned() {
                        map.insert(name, tn);
                    }
                }
            }
        }

        let _ = kw; // silence unused warning if config changes
    }
    map
}

fn collect_swift_extensions(
    items: &[ts_pack::StructureItem],
    map: &mut std::collections::HashMap<String, std::collections::HashSet<String>>,
) {
    for item in items {
        if item.kind == ts_pack::StructureKind::Extension {
            if let Some(type_name) = item.name.as_ref().and_then(|n| normalize_swift_type(n)) {
                let entry = map.entry(type_name).or_default();
                for child in &item.children {
                    if matches!(
                        child.kind,
                        ts_pack::StructureKind::Method | ts_pack::StructureKind::Function
                    ) {
                        if let Some(name) = child.name.as_ref() {
                            entry.insert(name.clone());
                        }
                    }
                }
            }
        }
        if !item.children.is_empty() {
            collect_swift_extensions(&item.children, map);
        }
    }
}

fn collect_swift_extension_spans(items: &[ts_pack::StructureItem], spans: &mut Vec<(usize, usize, String)>) {
    for item in items {
        if item.kind == ts_pack::StructureKind::Extension {
            if let Some(type_name) = item.name.as_ref().and_then(|n| normalize_swift_type(n)) {
                spans.push((item.span.start_byte, item.span.end_byte, type_name));
            }
        }
        if !item.children.is_empty() {
            collect_swift_extension_spans(&item.children, spans);
        }
    }
}

fn collect_swift_type_spans(items: &[ts_pack::StructureItem], spans: &mut Vec<(usize, usize, String)>) {
    for item in items {
        if matches!(
            item.kind,
            ts_pack::StructureKind::Class
                | ts_pack::StructureKind::Struct
                | ts_pack::StructureKind::Enum
                | ts_pack::StructureKind::Protocol
        ) {
            if let Some(name) = item.name.as_ref().and_then(|n| normalize_swift_type(n)) {
                spans.push((item.span.start_byte, item.span.end_byte, name));
            }
        }
        if !item.children.is_empty() {
            collect_swift_type_spans(&item.children, spans);
        }
    }
}

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

/// One inferred call edge (Swift extension resolution).
struct InferredCallRow {
    caller_id: String,
    callee: String,
    receiver_type: String,
    project_id: Arc<str>,
    caller_filepath: String,
    allow_same_file: bool,
}

struct DbEdgeRow {
    src: String,
    tgt: String,
}

struct DbModelEdgeRow {
    src: String,
    model: String,
    project_id: Arc<str>,
}

struct ExternalApiNode {
    id: String,
    url: String,
    project_id: Arc<str>,
}

struct ExternalApiEdgeRow {
    src: String,
    tgt: String,
}

struct ImportSymbolRequest {
    src_id: String,
    src_filepath: String,
    module: String,
    items: Vec<String>,
}

struct ImportSymbolEdgeRow {
    src: String,
    tgt: String,
}

struct ExportSymbolEdgeRow {
    src: String,
    tgt: String,
}

struct SwiftFileContext {
    file_id: String,
    filepath: String,
    symbol_spans: Vec<(usize, usize, String)>,
    extension_spans: Vec<(usize, usize, String)>,
    type_spans: Vec<(usize, usize, String)>,
    call_sites: Vec<tags::CallSite>,
    var_types: std::collections::HashMap<String, String>,
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

async fn write_inferred_calls(graph: &Arc<Graph>, batch: &[InferredCallRow]) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (caller:Node {id: item.caller_id}) \
         MATCH (callee:Node {project_id: item.pid, name: item.callee}) \
         WHERE (callee:Function OR callee:Class OR callee:Struct OR callee:Method) \
           AND callee.qualified_name IS NOT NULL \
           AND callee.qualified_name STARTS WITH item.recv + '.' \
           AND (item.allow_same_file = true OR callee.filepath <> item.caller_fp) \
         MERGE (caller)-[:CALLS_INFERRED]->(callee)"
            .to_string(),
    )
    .param("batch", bolt);
    let _ = graph.run(q).await;
}

async fn write_db_edges(graph: &Arc<Graph>, batch: &[DbEdgeRow]) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (a:File {id: item.src}) \
         MATCH (b:File {id: item.tgt}) \
         MERGE (a)-[:CALLS_DB]->(b)"
            .to_string(),
    )
    .param("batch", bolt);
    let _ = graph.run(q).await;
}

async fn write_db_model_edges(graph: &Arc<Graph>, batch: &[DbModelEdgeRow]) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MERGE (m:Model {id: item.pid + ':model:' + item.model}) \
         SET m.project_id = item.pid, m.name = item.model \
         WITH item, m \
         MATCH (a:File {id: item.src}) \
         MERGE (a)-[:CALLS_DB_MODEL]->(m)"
            .to_string(),
    )
    .param("batch", bolt);
    let _ = graph.run(q).await;
}

async fn write_external_api_nodes(graph: &Arc<Graph>, batch: &[ExternalApiNode]) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MERGE (e:ExternalAPI {id: item.id}) \
         SET e.project_id = item.pid, \
             e.url = item.url"
            .to_string(),
    )
    .param("batch", bolt);
    let _ = graph.run(q).await;
}

async fn write_external_api_edges(graph: &Arc<Graph>, batch: &[ExternalApiEdgeRow]) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (a:File {id: item.src}) \
         MATCH (b:ExternalAPI {id: item.tgt}) \
         MERGE (a)-[:CALLS_API_EXTERNAL]->(b)"
            .to_string(),
    )
    .param("batch", bolt);
    let _ = graph.run(q).await;
}

async fn write_import_symbol_edges(graph: &Arc<Graph>, batch: &[ImportSymbolEdgeRow]) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (a:File {id: item.src}) \
         MATCH (b:Node {id: item.tgt}) \
         MERGE (a)-[:IMPORTS_SYMBOL]->(b)"
            .to_string(),
    )
    .param("batch", bolt);
    let _ = graph.run(q).await;
}

async fn write_export_symbol_edges(graph: &Arc<Graph>, batch: &[ExportSymbolEdgeRow]) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (a:File {id: item.src}) \
         MATCH (b:Node {id: item.tgt}) \
         MERGE (a)-[:EXPORTS_SYMBOL]->(b)"
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
    swift_extensions: Option<std::collections::HashMap<String, std::collections::HashSet<String>>>,
    swift_context: Option<SwiftFileContext>,
    db_delegates: Vec<String>,
    external_urls: Vec<String>,
    import_symbol_requests: Vec<ImportSymbolRequest>,
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
    let mut inferred_call_rows: Vec<InferredCallRow> = Vec::new();
    let mut swift_extension_map: std::collections::HashMap<String, std::collections::HashSet<String>> =
        std::collections::HashMap::new();
    let mut swift_contexts: Vec<SwiftFileContext> = Vec::new();
    let mut db_sources: Vec<String> = Vec::new();
    let mut db_delegates_by_file: Vec<(String, String)> = Vec::new();
    let mut external_api_edges: Vec<ExternalApiEdgeRow> = Vec::new();
    let mut external_api_urls: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut seen_external_edges: std::collections::HashSet<(String, String)> = std::collections::HashSet::new();
    let mut import_symbol_edges: Vec<ImportSymbolEdgeRow> = Vec::new();
    let mut export_symbol_edges: Vec<ExportSymbolEdgeRow> = Vec::new();
    let mut seen_import_symbol: std::collections::HashSet<(String, String)> = std::collections::HashSet::new();
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
            let mut swift_extensions: Option<std::collections::HashMap<String, std::collections::HashSet<String>>> =
                None;
            let mut swift_context: Option<SwiftFileContext> = None;

            // Build exported-name set from structural result + tags visibility
            let mut exported_names: std::collections::HashSet<String> =
                result.exports.iter().map(|e| e.name.clone()).collect();
            let tags_result = ts_pack::parse_string(lang_name, source.as_bytes())
                .ok()
                .and_then(|tree| tags::run_tags(lang_name, &tree, source.as_bytes()));

            // --- Consume tags result: split into exported names + call sites ---
            let (tag_exported, raw_call_sites, db_delegates, external_calls, const_strings) = match tags_result {
                Some(tr) => (
                    tr.exported_names,
                    tr.call_sites,
                    tr.db_delegates,
                    tr.external_calls,
                    tr.const_strings,
                ),
                None => (
                    std::collections::HashSet::new(),
                    Vec::new(),
                    std::collections::HashSet::new(),
                    Vec::new(),
                    std::collections::HashMap::new(),
                ),
            };
            exported_names.extend(tag_exported);
            let call_sites = raw_call_sites;
            let db_delegates = db_delegates.into_iter().collect::<Vec<_>>();
            let mut external_urls: Vec<String> = Vec::new();
            for call in external_calls {
                let url = match call.arg {
                    tags::ExternalCallArg::Literal(value) => Some(value),
                    tags::ExternalCallArg::Identifier(name) => const_strings.get(&name).cloned(),
                    tags::ExternalCallArg::ConcatIdentLiteral { ident, literal } => {
                        const_strings.get(&ident).map(|base| format!("{base}{literal}"))
                    }
                    tags::ExternalCallArg::ConcatLiteralIdent { literal, ident } => {
                        const_strings.get(&ident).map(|base| format!("{literal}{base}"))
                    }
                    tags::ExternalCallArg::UrlLiteral { path, base } => join_url(&base, &path),
                    tags::ExternalCallArg::UrlWithBaseIdent { path, base_ident } => {
                        const_strings.get(&base_ident).and_then(|base| join_url(base, &path))
                    }
                };
                if let Some(url) = url {
                    if url.starts_with("http://") || url.starts_with("https://") || url.starts_with("env://") {
                        external_urls.push(url);
                    }
                }
            }
            let is_backend = rel_path.starts_with("src/api/")
                || rel_path.starts_with("src/services/")
                || rel_path.starts_with("src/webhooks/")
                || rel_path.starts_with("src/jobs/")
                || rel_path.starts_with("src/db/")
                || rel_path.starts_with("src/seed/")
                || rel_path == "src/server.ts";
            let is_public = rel_path.starts_with("src/public/");
            let is_backend = is_backend && !is_public;

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

            let symbol_calls: Vec<SymbolCallRow> = call_sites
                .clone()
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

            if lang_name == "swift" {
                let mut ext_map: std::collections::HashMap<String, std::collections::HashSet<String>> =
                    std::collections::HashMap::new();
                collect_swift_extensions(&result.structure, &mut ext_map);
                if !ext_map.is_empty() {
                    swift_extensions = Some(ext_map);
                }

                let mut ext_spans: Vec<(usize, usize, String)> = Vec::new();
                collect_swift_extension_spans(&result.structure, &mut ext_spans);

                let mut type_spans: Vec<(usize, usize, String)> = Vec::new();
                collect_swift_type_spans(&result.structure, &mut type_spans);

                let var_types = parse_swift_var_types(&source);
                if !var_types.is_empty() {
                    swift_context = Some(SwiftFileContext {
                        file_id: file_id.clone(),
                        filepath: rel_path.clone(),
                        symbol_spans: symbol_spans.clone(),
                        extension_spans: ext_spans.clone(),
                        type_spans: type_spans.clone(),
                        call_sites: call_sites.clone(),
                        var_types,
                    });
                } else if !call_sites.is_empty() {
                    swift_context = Some(SwiftFileContext {
                        file_id: file_id.clone(),
                        filepath: rel_path.clone(),
                        symbol_spans: symbol_spans.clone(),
                        extension_spans: ext_spans.clone(),
                        type_spans: type_spans.clone(),
                        call_sites: call_sites.clone(),
                        var_types: std::collections::HashMap::new(),
                    });
                }
            }

            let mut import_symbol_requests: Vec<ImportSymbolRequest> = Vec::new();
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

                if !imp.items.is_empty() && !imp.is_wildcard {
                    import_symbol_requests.push(ImportSymbolRequest {
                        src_id: file_id.clone(),
                        src_filepath: rel_path.clone(),
                        module: imp.source.clone(),
                        items: imp.items.clone(),
                    });
                }
            }

            Some(FileResult {
                file_node,
                symbols,
                relations,
                imports,
                import_rels,
                symbol_calls,
                swift_extensions,
                swift_context,
                db_delegates: if is_backend { db_delegates } else { Vec::new() },
                external_urls,
                import_symbol_requests,
            })
        };

        let batch_results: Vec<FileResult> = if std::env::var("TS_PACK_SERIAL_PARSE").is_ok() {
            batch.iter().filter_map(parse_entry).collect()
        } else {
            batch.par_iter().filter_map(parse_entry).collect()
        };

        // Merge batch results into global reservoirs
        for res in batch_results {
            let file_id = res.file_node.id.clone();
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
            if let Some(exts) = res.swift_extensions {
                for (ty, methods) in exts {
                    swift_extension_map.entry(ty).or_default().extend(methods);
                }
            }
            if let Some(ctx) = res.swift_context {
                swift_contexts.push(ctx);
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
    for syms in all_symbols.values() {
        for sym in syms {
            symbols_by_file
                .entry(sym.filepath.clone())
                .or_default()
                .insert(sym.name.clone(), sym.id.clone());
        }
    }
    let files_set: HashSet<String> = all_files.iter().map(|f| f.filepath.clone()).collect();
    for req in &import_symbol_requests {
        let target_fp = match resolve_module_path(&req.src_filepath, &req.module, &files_set) {
            Some(fp) => fp,
            None => continue,
        };
        let Some(sym_map) = symbols_by_file.get(&target_fp) else {
            continue;
        };
        for item in &req.items {
            let name = clean_import_name(item);
            if name.is_empty() {
                continue;
            }
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

    if !inferred_call_rows.is_empty() {
        let t_inf = Instant::now();
        let inf_count = inferred_call_rows.len();
        stream::iter(inferred_call_rows.chunks(CALLS_BATCH_SIZE))
            .for_each_concurrent(REL_CONCURRENCY, |chunk| {
                let g = Arc::clone(&graph);
                async move { write_inferred_calls(&g, chunk).await }
            })
            .await;
        eprintln!(
            "[ts-pack-index] CALLS_INFERRED writes done in {:.2}s (rows={})",
            t_inf.elapsed().as_secs_f64(),
            inf_count,
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

    let indexed_paths: Vec<PathBuf> = manifest.into_iter().map(|m| PathBuf::from(m.abs_path)).collect();

    Ok(indexed_paths)
}
