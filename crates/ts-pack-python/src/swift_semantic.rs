use neo4rs::{query, ConfigBuilder, Graph};
use serde_json::{json, Map, Value};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;

#[derive(Clone, Debug)]
struct SwiftSymbolRecord {
    name: String,
    base_name: String,
    kind: String,
    start_line: usize,
    end_line: usize,
    usr: Option<String>,
    doc_comment: Option<String>,
    inherited_types: Vec<String>,
}

#[derive(Clone, Debug)]
struct GraphSymbolRow {
    sid: String,
    name: String,
    start_line: usize,
    end_line: usize,
}

fn which_binary(name: &str) -> Option<String> {
    if let Some(paths) = std::env::var_os("PATH") {
        for path in std::env::split_paths(&paths) {
            let candidate = path.join(name);
            if candidate.is_file() {
                return Some(candidate.to_string_lossy().into_owned());
            }
        }
    }
    if let Ok(py) = std::env::current_exe()
        && let Some(py_dir) = py.parent()
    {
        let candidate = py_dir.join(name);
        if candidate.is_file() {
            return Some(candidate.to_string_lossy().into_owned());
        }
    }
    for path in [
        format!("/opt/homebrew/bin/{name}"),
        format!("/usr/local/bin/{name}"),
        format!("/usr/bin/{name}"),
    ] {
        let candidate = Path::new(&path);
        if candidate.is_file() {
            return Some(path);
        }
    }
    None
}

fn line_number(raw: &[u8], offset: usize) -> usize {
    raw[..offset.min(raw.len())].iter().filter(|&&b| b == b'\n').count() + 1
}

fn clean_name(value: &str) -> String {
    value.trim().to_string()
}

fn base_name(name: &str) -> String {
    clean_name(name).split('(').next().unwrap_or("").trim().to_string()
}

fn extract_preceding_doc_comment(lines: &[&str], start_line: usize) -> Option<String> {
    if start_line <= 1 {
        return None;
    }
    let mut idx = start_line.saturating_sub(2);
    loop {
        if idx >= lines.len() {
            return None;
        }
        if !lines[idx].trim().is_empty() {
            break;
        }
        if idx == 0 {
            return None;
        }
        idx -= 1;
    }
    let line = lines[idx].trim();
    if line.starts_with("///") {
        let mut collected: Vec<String> = Vec::new();
        let mut cursor = idx as isize;
        while cursor >= 0 {
            let current = lines[cursor as usize].trim();
            if !current.starts_with("///") {
                break;
            }
            collected.push(current[3..].trim_start().to_string());
            cursor -= 1;
        }
        collected.reverse();
        let text = collected.join("\n").trim().to_string();
        return if text.is_empty() { None } else { Some(text) };
    }
    if line.ends_with("*/") {
        let mut collected: Vec<String> = Vec::new();
        let mut cursor = idx as isize;
        while cursor >= 0 {
            let current = lines[cursor as usize].trim_end().to_string();
            let contains_start = current.contains("/**");
            collected.push(current);
            if contains_start {
                break;
            }
            cursor -= 1;
        }
        if collected.last().map(|s| s.contains("/**")).unwrap_or(false) {
            collected.reverse();
            let mut normalized: Vec<String> = Vec::new();
            for part in collected {
                let mut piece = part.trim().to_string();
                if let Some(rest) = piece.strip_prefix("/**") {
                    piece = rest.trim().to_string();
                }
                if let Some(rest) = piece.strip_suffix("*/") {
                    piece = rest.trim().to_string();
                }
                if let Some(rest) = piece.strip_prefix('*') {
                    piece = rest.trim_start().to_string();
                }
                if !piece.is_empty() {
                    normalized.push(piece);
                }
            }
            let text = normalized.join("\n").trim().to_string();
            return if text.is_empty() { None } else { Some(text) };
        }
    }
    None
}

fn clean_inherited_type_name(value: &str) -> String {
    let mut cleaned = clean_name(value);
    if cleaned.is_empty() {
        return cleaned;
    }
    if let Some((head, _)) = cleaned.split_once('<') {
        cleaned = head.trim().to_string();
    }
    if let Some((_, tail)) = cleaned.rsplit_once('.') {
        cleaned = tail.trim().to_string();
    }
    if let Some((head, _)) = cleaned.split_once(':') {
        cleaned = head.trim().to_string();
    }
    if let Some((head, _)) = cleaned.split_once('&') {
        cleaned = head.trim().to_string();
    }
    cleaned
}

pub(crate) fn json_to_bolt(v: Value) -> neo4rs::BoltType {
    match v {
        Value::String(s) => neo4rs::BoltType::from(s),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                neo4rs::BoltType::from(i)
            } else if let Some(f) = n.as_f64() {
                neo4rs::BoltType::from(f)
            } else {
                neo4rs::BoltType::from(0i64)
            }
        }
        Value::Bool(b) => neo4rs::BoltType::from(b),
        Value::Null => neo4rs::BoltType::Null(neo4rs::BoltNull),
        Value::Array(arr) => neo4rs::BoltType::from(arr.into_iter().map(json_to_bolt).collect::<Vec<_>>()),
        Value::Object(map) => {
            let mut bolt_map = HashMap::new();
            for (k, val) in map {
                bolt_map.insert(k, json_to_bolt(val));
            }
            neo4rs::BoltType::from(bolt_map)
        }
    }
}

fn match_symbol_record(record: &SwiftSymbolRecord, symbols: &[GraphSymbolRow]) -> Option<GraphSymbolRow> {
    let record_name = clean_name(&record.name);
    let record_base = clean_name(&record.base_name);
    let mut candidates: Vec<(usize, usize, usize, usize, GraphSymbolRow)> = Vec::new();
    for sym in symbols {
        let sym_name = clean_name(&sym.name);
        let sym_base = base_name(&sym_name);
        if record_base != sym_base && record_name != sym_name {
            continue;
        }
        let overlap = std::cmp::min(record.end_line, sym.end_line) as isize
            - std::cmp::max(record.start_line, sym.start_line) as isize;
        let distance = record.start_line.abs_diff(sym.start_line) + record.end_line.abs_diff(sym.end_line);
        candidates.push((
            if record_name == sym_name { 0 } else { 1 },
            if overlap >= 0 { 0 } else { 1 },
            distance,
            sym.start_line,
            sym.clone(),
        ));
    }
    candidates.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)).then(a.2.cmp(&b.2)).then(a.3.cmp(&b.3)));
    candidates.into_iter().next().map(|entry| entry.4)
}

fn structure_records_from_value(data: &Value, raw: &[u8], lines: &[&str], out: &mut Vec<SwiftSymbolRecord>) {
    let items = data
        .get("key.substructure")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    for item in items {
        let kind = clean_name(item.get("key.kind").and_then(Value::as_str).unwrap_or(""));
        let name = clean_name(item.get("key.name").and_then(Value::as_str).unwrap_or(""));
        let offset = item.get("key.offset").and_then(Value::as_u64).unwrap_or(0) as usize;
        let length = item.get("key.length").and_then(Value::as_u64).unwrap_or(0) as usize;
        let start_line = line_number(raw, offset);
        let end_line = line_number(raw, offset.saturating_add(length));
        let inherited_types = item
            .get("key.inheritedtypes")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|entry| entry.get("key.name").and_then(Value::as_str))
            .map(clean_inherited_type_name)
            .filter(|v| !v.is_empty())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        if kind.starts_with("source.lang.swift.decl") && !name.is_empty() {
            let doc_comment = clean_name(item.get("key.doc.comment").and_then(Value::as_str).unwrap_or(""));
            out.push(SwiftSymbolRecord {
                name: name.clone(),
                base_name: base_name(&name),
                kind,
                start_line,
                end_line,
                usr: {
                    let usr = clean_name(item.get("key.usr").and_then(Value::as_str).unwrap_or(""));
                    if usr.is_empty() { None } else { Some(usr) }
                },
                doc_comment: if !doc_comment.is_empty() {
                    Some(doc_comment)
                } else {
                    extract_preceding_doc_comment(lines, start_line)
                },
                inherited_types,
            });
        }
        structure_records_from_value(&item, raw, lines, out);
    }
}

fn extract_swift_structure_records(sourcekitten: &str, file_path: &Path) -> Vec<SwiftSymbolRecord> {
    let raw = match fs::read(file_path) {
        Ok(raw) => raw,
        Err(_) => return Vec::new(),
    };
    let output = match Command::new(sourcekitten)
        .args(["structure", "--file"])
        .arg(file_path)
        .output()
    {
        Ok(output) if output.status.success() => output,
        _ => return Vec::new(),
    };
    let data: Value = match serde_json::from_slice(&output.stdout) {
        Ok(data) => data,
        Err(_) => return Vec::new(),
    };
    let text = String::from_utf8_lossy(&raw);
    let lines = text.lines().collect::<Vec<_>>();
    let mut records = Vec::new();
    structure_records_from_value(&data, &raw, &lines, &mut records);
    records
}

fn clean_path_list(value: &Value) -> Vec<String> {
    match value {
        Value::Null => Vec::new(),
        Value::String(s) => s.split_whitespace().map(|part| part.to_string()).collect(),
        Value::Array(items) => items
            .iter()
            .filter_map(Value::as_str)
            .map(|part| part.to_string())
            .collect(),
        _ => Vec::new(),
    }
}

fn clean_define_list(value: &Value) -> Vec<String> {
    match value {
        Value::Null => Vec::new(),
        Value::String(s) => s.split_whitespace().map(|part| part.to_string()).collect(),
        Value::Array(items) => items
            .iter()
            .filter_map(Value::as_str)
            .map(|part| part.trim().to_string())
            .filter(|part| !part.is_empty())
            .collect(),
        _ => Vec::new(),
    }
}

fn xcode_build_settings(xcodebuild: &str, project_file: &Path, scheme_name: &str) -> Vec<Value> {
    let project_bundle = if project_file.file_name().and_then(|n| n.to_str()) == Some("project.pbxproj") {
        project_file.parent().map(PathBuf::from).unwrap_or_else(|| project_file.to_path_buf())
    } else {
        project_file.to_path_buf()
    };
    let output = match Command::new(xcodebuild)
        .args([
            "-project",
            &project_bundle.to_string_lossy(),
            "-scheme",
            scheme_name,
            "-destination",
            "platform=macOS",
            "-showBuildSettings",
            "-json",
        ])
        .output()
    {
        Ok(output) if output.status.success() => output,
        _ => return Vec::new(),
    };
    serde_json::from_slice(&output.stdout).unwrap_or_default()
}

fn compiler_args_from_build_settings(build_settings: &Map<String, Value>) -> Vec<String> {
    let mut args = Vec::new();
    if let Some(sdkroot) = build_settings.get("SDKROOT").and_then(Value::as_str)
        && !sdkroot.trim().is_empty()
    {
        args.push("-sdk".to_string());
        args.push(sdkroot.to_string());
    }
    let module_name = build_settings
        .get("PRODUCT_MODULE_NAME")
        .and_then(Value::as_str)
        .or_else(|| build_settings.get("TARGET_NAME").and_then(Value::as_str));
    if let Some(module_name) = module_name
        && !module_name.trim().is_empty()
    {
        args.push("-module-name".to_string());
        args.push(module_name.to_string());
    }
    for define in clean_define_list(build_settings.get("SWIFT_ACTIVE_COMPILATION_CONDITIONS").unwrap_or(&Value::Null)) {
        args.push("-D".to_string());
        args.push(define);
    }
    for path in clean_path_list(build_settings.get("FRAMEWORK_SEARCH_PATHS").unwrap_or(&Value::Null)) {
        args.push("-F".to_string());
        args.push(path);
    }
    for path in clean_path_list(build_settings.get("HEADER_SEARCH_PATHS").unwrap_or(&Value::Null)) {
        args.push("-I".to_string());
        args.push(path);
    }
    for path in clean_path_list(build_settings.get("SWIFT_INCLUDE_PATHS").unwrap_or(&Value::Null)) {
        args.push("-I".to_string());
        args.push(path);
    }
    args.extend(clean_path_list(build_settings.get("OTHER_SWIFT_FLAGS").unwrap_or(&Value::Null)));
    args
}

fn collect_xcode_projects(root: &Path, out: &mut Vec<PathBuf>) {
    let read_dir = match fs::read_dir(root) {
        Ok(read_dir) => read_dir,
        Err(_) => return,
    };
    for entry in read_dir.flatten() {
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if path.is_dir() {
            if matches!(name.as_ref(), ".git" | ".build" | "build" | "DerivedData") {
                continue;
            }
            if path.extension().and_then(|ext| ext.to_str()) == Some("xcodeproj") {
                let pbxproj = path.join("project.pbxproj");
                if pbxproj.is_file() {
                    out.push(pbxproj);
                }
                continue;
            }
            collect_xcode_projects(&path, out);
        }
    }
}

fn candidate_xcode_projects(project_root: &Path) -> Vec<PathBuf> {
    let mut projects = Vec::new();
    collect_xcode_projects(project_root, &mut projects);
    projects.sort();
    projects.dedup();
    projects
}

fn collect_swift_files(root: &Path, out: &mut Vec<PathBuf>) {
    let read_dir = match fs::read_dir(root) {
        Ok(read_dir) => read_dir,
        Err(_) => return,
    };
    for entry in read_dir.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_swift_files(&path, out);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("swift") {
            out.push(path);
        }
    }
}

fn target_swift_files(project_root: &Path, target_name: &str) -> Vec<PathBuf> {
    let target_dir = project_root.join(target_name);
    if !target_dir.is_dir() {
        return Vec::new();
    }
    let mut files = Vec::new();
    collect_swift_files(&target_dir, &mut files);
    files.sort();
    files
}

fn semantic_index_records(
    sourcekitten: &str,
    file_path: &Path,
    compiler_args: &[String],
    target_files: &[PathBuf],
) -> Vec<SwiftSymbolRecord> {
    if target_files.is_empty() {
        return Vec::new();
    }
    let mut cmd = Command::new(sourcekitten);
    cmd.args(["index", "--file"]);
    cmd.arg(file_path);
    cmd.arg("--");
    cmd.args(compiler_args);
    cmd.args(target_files.iter().map(|path| path.as_os_str()));
    let output = match cmd.output() {
        Ok(output) if output.status.success() => output,
        _ => return Vec::new(),
    };
    let data: Value = match serde_json::from_slice(&output.stdout) {
        Ok(data) => data,
        Err(_) => return Vec::new(),
    };
    let mut records = Vec::new();
    let mut seen = HashSet::new();
    let mut stack = vec![data];
    while let Some(item) = stack.pop() {
        match item {
            Value::Object(map) => {
                let name = clean_name(map.get("key.name").and_then(Value::as_str).unwrap_or(""));
                let usr = clean_name(map.get("key.usr").and_then(Value::as_str).unwrap_or(""));
                if !name.is_empty() && !usr.is_empty() && seen.insert((name.clone(), usr.clone())) {
                    records.push(SwiftSymbolRecord {
                        name: name.clone(),
                        base_name: base_name(&name),
                        kind: clean_name(map.get("key.kind").and_then(Value::as_str).unwrap_or("")),
                        start_line: 0,
                        end_line: 0,
                        usr: Some(usr),
                        doc_comment: None,
                        inherited_types: Vec::new(),
                    });
                }
                for value in map.into_values() {
                    if matches!(value, Value::Array(_) | Value::Object(_)) {
                        stack.push(value);
                    }
                }
            }
            Value::Array(items) => stack.extend(items),
            _ => {}
        }
    }
    records
}

pub fn extract_swift_semantic_facts_value(project_path: &str) -> Value {
    let sourcekitten = match which_binary("sourcekitten") {
        Some(path) => path,
        None => return json!({}),
    };
    let xcodebuild = match which_binary("xcodebuild") {
        Some(path) => path,
        None => return json!({}),
    };
    let project_root = Path::new(project_path);
    let mut out = Map::new();

    for project_file in candidate_xcode_projects(project_root) {
        let scheme_name = match project_file.parent().and_then(|p| p.file_stem()).and_then(|s| s.to_str()) {
            Some(name) if !name.is_empty() => name.to_string(),
            _ => continue,
        };
        for entry in xcode_build_settings(&xcodebuild, &project_file, &scheme_name) {
            let build_settings = match entry.get("buildSettings").and_then(Value::as_object) {
                Some(settings) => settings,
                None => continue,
            };
            let target_name = entry
                .get("target")
                .and_then(Value::as_str)
                .or_else(|| build_settings.get("TARGET_NAME").and_then(Value::as_str));
            let Some(target_name) = target_name.filter(|name| !name.trim().is_empty()) else {
                continue;
            };
            let target_files = target_swift_files(project_root, target_name);
            if target_files.is_empty() {
                continue;
            }
            let compiler_args = compiler_args_from_build_settings(build_settings);
            if compiler_args.is_empty() {
                continue;
            }
            for abs_path in &target_files {
                let rel_path = match abs_path.strip_prefix(project_root) {
                    Ok(path) => path.to_string_lossy().replace('\\', "/"),
                    Err(_) => continue,
                };
                let structure_records = extract_swift_structure_records(&sourcekitten, abs_path);
                let semantic_records = semantic_index_records(&sourcekitten, abs_path, &compiler_args, &target_files);
                let mut semantic_usr_by_base_name = HashMap::new();
                for record in semantic_records {
                    if !record.base_name.is_empty()
                        && let Some(usr) = record.usr
                    {
                        semantic_usr_by_base_name.entry(record.base_name).or_insert(usr);
                    }
                }
                let merged = structure_records
                    .into_iter()
                    .map(|record| {
                        json!({
                            "filepath": rel_path,
                            "name": record.name,
                            "base_name": record.base_name,
                            "kind": record.kind,
                            "start_line": record.start_line,
                            "end_line": record.end_line,
                            "usr": semantic_usr_by_base_name.get(&record.base_name).cloned().or(record.usr),
                            "doc_comment": record.doc_comment,
                            "inherited_types": record.inherited_types,
                        })
                    })
                    .collect::<Vec<_>>();
                if !merged.is_empty() {
                    out.insert(rel_path, Value::Array(merged));
                }
            }
        }
    }

    Value::Object(out)
}

async fn load_swift_symbols(
    graph: &Arc<Graph>,
    project_id: &str,
    filepaths: &[String],
) -> Result<HashMap<String, Vec<GraphSymbolRow>>, Box<dyn std::error::Error>> {
    let mut result = graph
        .execute(
            query(
                "MATCH (f:File {project_id:$pid})-[:CONTAINS]->(s:Node)
                 WHERE f.filepath IN $fps
                 RETURN f.filepath AS filepath,
                        s.id AS sid,
                        s.name AS name,
                        s.start_line AS start_line,
                        s.end_line AS end_line
                 ORDER BY filepath, start_line, end_line, name",
            )
            .param("pid", project_id.to_string())
            .param(
                "fps",
                neo4rs::BoltType::from(
                    filepaths
                        .iter()
                        .cloned()
                        .map(neo4rs::BoltType::from)
                        .collect::<Vec<_>>(),
                ),
            ),
        )
        .await?;

    let mut rows_by_file: HashMap<String, Vec<GraphSymbolRow>> = HashMap::new();
    while let Some(row) = result.next().await? {
        let filepath: String = row.get("filepath").unwrap_or_default();
        let sid: String = row.get("sid").unwrap_or_default();
        let name: String = row.get("name").unwrap_or_default();
        let start_line: i64 = row.get("start_line").unwrap_or(0);
        let end_line: i64 = row.get("end_line").unwrap_or(0);
        rows_by_file.entry(filepath).or_default().push(GraphSymbolRow {
            sid,
            name,
            start_line: start_line.max(0) as usize,
            end_line: end_line.max(0) as usize,
        });
    }
    Ok(rows_by_file)
}

async fn write_swift_enrichment(
    graph: &Arc<Graph>,
    project_id: &str,
    rows: &[Value],
) -> Result<(), Box<dyn std::error::Error>> {
    if rows.is_empty() {
        return Ok(());
    }
    graph.run(
        query(
            "UNWIND $rows AS row
             MATCH (s:Node {project_id:$pid, id:row.sid})
             SET s.swift_sourcekitten = true,
                 s.swift_sourcekitten_kind = row.kind,
                 s.swift_usr = CASE
                     WHEN row.usr IS NOT NULL AND trim(row.usr) <> '' THEN row.usr
                     ELSE s.swift_usr
                 END,
                 s.swift_inherited_types = row.inherited_types,
                 s.doc_comment = CASE
                     WHEN (s.doc_comment IS NULL OR trim(s.doc_comment) = '')
                          AND row.doc_comment IS NOT NULL
                          AND trim(row.doc_comment) <> ''
                     THEN row.doc_comment
                     ELSE s.doc_comment
                 END,
                 s.swift_doc_comment = CASE
                     WHEN row.doc_comment IS NOT NULL AND trim(row.doc_comment) <> ''
                     THEN row.doc_comment
                     ELSE s.swift_doc_comment
                 END
             FOREACH (_ IN CASE WHEN row.inherited_types IS NULL OR size(row.inherited_types) = 0 THEN [] ELSE [1] END |
                 FOREACH (type_name IN row.inherited_types |
                     MERGE (t:SwiftTypeRef {project_id:$pid, name:type_name})
                     MERGE (s)-[:SWIFT_INHERITS_TYPE]->(t)
                 )
             )",
        )
        .param("pid", project_id.to_string())
        .param(
            "rows",
            neo4rs::BoltType::from(rows.iter().cloned().map(json_to_bolt).collect::<Vec<_>>()),
        ),
    )
    .await?;
    Ok(())
}

pub async fn enrich_swift_graph_async(
    project_path: &str,
    project_id: &str,
    indexed_files: &[String],
    neo4j_uri: &str,
    neo4j_user: &str,
    neo4j_pass: &str,
    neo4j_db: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    let enabled = matches!(
        std::env::var("LM_PROXY_SWIFT_SOURCEKITTEN")
            .unwrap_or_default()
            .trim()
            .to_ascii_lowercase()
            .as_str(),
        "1" | "true" | "yes" | "on"
    );
    if !enabled {
        return Ok(json!({"enabled": false, "files": 0, "symbols": 0}));
    }

    let root = std::fs::canonicalize(project_path).unwrap_or_else(|_| PathBuf::from(project_path));
    let swift_abs_paths = indexed_files
        .iter()
        .map(PathBuf::from)
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("swift"))
        .filter(|path| path.is_file())
        .collect::<Vec<_>>();
    if swift_abs_paths.is_empty() {
        return Ok(json!({"enabled": true, "available": true, "files": 0, "symbols": 0}));
    }

    let semantic_records = extract_swift_semantic_facts_value(project_path);
    let Value::Object(semantic_records) = semantic_records else {
        return Ok(json!({"enabled": true, "available": false, "files": 0, "symbols": 0}));
    };
    if semantic_records.is_empty() {
        return Ok(json!({"enabled": true, "available": false, "files": 0, "symbols": 0}));
    }

    let filepaths = swift_abs_paths
        .iter()
        .filter_map(|path| path.strip_prefix(&root).ok())
        .map(|path| path.to_string_lossy().replace('\\', "/"))
        .collect::<Vec<_>>();

    let neo4j_config = ConfigBuilder::default()
        .uri(neo4j_uri)
        .user(neo4j_user)
        .password(neo4j_pass)
        .db(neo4j_db)
        .max_connections(8)
        .fetch_size(500)
        .build()?;
    let graph = Arc::new(Graph::connect(neo4j_config).await?);
    let graph_symbols = load_swift_symbols(&graph, project_id, &filepaths).await?;

    let mut updates = Vec::new();
    let mut files_with_matches = 0usize;
    for rel_path in &filepaths {
        let Some(records) = semantic_records.get(rel_path).and_then(Value::as_array) else {
            continue;
        };
        let symbols = graph_symbols.get(rel_path).cloned().unwrap_or_default();
        let mut matched_here = 0usize;
        for record in records {
            let item = SwiftSymbolRecord {
                name: clean_name(record.get("name").and_then(Value::as_str).unwrap_or("")),
                base_name: clean_name(record.get("base_name").and_then(Value::as_str).unwrap_or("")),
                kind: clean_name(record.get("kind").and_then(Value::as_str).unwrap_or("")),
                start_line: record.get("start_line").and_then(Value::as_u64).unwrap_or(0) as usize,
                end_line: record.get("end_line").and_then(Value::as_u64).unwrap_or(0) as usize,
                usr: record.get("usr").and_then(Value::as_str).map(str::to_string),
                doc_comment: record.get("doc_comment").and_then(Value::as_str).map(str::to_string),
                inherited_types: record
                    .get("inherited_types")
                    .and_then(Value::as_array)
                    .map(|items| {
                        items
                            .iter()
                            .filter_map(Value::as_str)
                            .map(str::to_string)
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default(),
            };
            let Some(matched_symbol) = match_symbol_record(&item, &symbols) else {
                continue;
            };
            matched_here += 1;
            updates.push(json!({
                "sid": matched_symbol.sid,
                "kind": item.kind,
                "usr": item.usr,
                "doc_comment": item.doc_comment,
                "inherited_types": item.inherited_types,
            }));
        }
        if matched_here > 0 {
            files_with_matches += 1;
        }
    }

    write_swift_enrichment(&graph, project_id, &updates).await?;
    Ok(json!({
        "enabled": true,
        "available": true,
        "files": files_with_matches,
        "symbols": updates.len(),
    }))
}
