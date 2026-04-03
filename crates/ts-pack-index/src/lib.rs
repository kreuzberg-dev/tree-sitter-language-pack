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

// ---------------------------------------------------------------------------
// Clone grouping (winnow) defaults
// ---------------------------------------------------------------------------

const WINNOW_MIN_TOKENS: usize = 20;
const WINNOW_MIN_FINGERPRINTS: usize = 12;
const WINNOW_BUCKET_LIMIT: usize = 40;
const WINNOW_FALLBACK_HASHES: usize = 6;
const WINNOW_FORCE_ALL_HASHES_MAX_FPS: usize = 25;
const WINNOW_MIN_OVERLAP: f64 = 0.6;
const WINNOW_TOKEN_SIM_THRESHOLD: f64 = 0.65;
const WINNOW_KGRAM_SIM_THRESHOLD: f64 = 0.7;
const WINNOW_MIN_SCORE: f64 = 0.85;
const WINNOW_SMALL_TOKEN_THRESHOLD: usize = 50;

const WINNOW_SMALL_K: usize = 5;
const WINNOW_SMALL_W: usize = 3;
const WINNOW_MEDIUM_K: usize = 9;
const WINNOW_MEDIUM_W: usize = 5;
const WINNOW_LARGE_K: usize = 15;
const WINNOW_LARGE_W: usize = 7;

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

fn project_root_from_manifest(manifest: &[ManifestEntry]) -> Option<String> {
    let first = manifest.first()?;
    if let Some(root) = first.abs_path.strip_suffix(&first.rel_path) {
        return Some(root.trim_end_matches('/').to_string());
    }
    let path = PathBuf::from(&first.abs_path);
    path.parent().and_then(|p| p.to_str()).map(|p| p.to_string())
}

fn extract_string_value(block: &str, key: &str) -> Option<String> {
    let idx = block.find(key)?;
    let rest = &block[idx + key.len()..];
    let quote_start = rest.find('"')?;
    let rest = &rest[quote_start + 1..];
    let quote_end = rest.find('"')?;
    Some(rest[..quote_end].to_string())
}

fn extract_string_array(block: &str, key: &str) -> Vec<String> {
    let mut out = Vec::new();
    let idx = match block.find(key) {
        Some(idx) => idx,
        None => return out,
    };
    let rest = &block[idx + key.len()..];
    let start = match rest.find('[') {
        Some(pos) => pos,
        None => return out,
    };
    let rest = &rest[start + 1..];
    let end = match rest.find(']') {
        Some(pos) => pos,
        None => return out,
    };
    let inner = &rest[..end];
    for part in inner.split(',') {
        let trimmed = part.trim();
        if trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2 {
            out.push(trimmed[1..trimmed.len() - 1].to_string());
        }
    }
    out
}

fn extract_pbx_value(block: &str, key: &str) -> Option<String> {
    let needle = format!("{key} =");
    let idx = block.find(&needle)?;
    let rest = block[idx + needle.len()..].trim_start();
    let end = rest.find(';')?;
    let raw = rest[..end].trim();
    Some(raw.trim_matches('"').to_string())
}

fn extract_pbx_id_array(block: &str, key: &str) -> Vec<String> {
    let needle = format!("{key} =");
    let idx = match block.find(&needle) {
        Some(idx) => idx,
        None => return Vec::new(),
    };
    let rest = &block[idx + needle.len()..];
    let start = match rest.find('(') {
        Some(pos) => pos,
        None => return Vec::new(),
    };
    let rest = &rest[start + 1..];
    let end = match rest.find(')') {
        Some(pos) => pos,
        None => return Vec::new(),
    };
    let inner = &rest[..end];
    let mut out = Vec::new();
    for part in inner.split(',') {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }
        let id = trimmed.split_whitespace().next().unwrap_or("");
        if !id.is_empty() {
            out.push(id.to_string());
        }
    }
    out
}

fn normalize_pbx_path(value: &str) -> String {
    value.trim().trim_matches('"').replace('\\', "/")
}

fn insert_module_files(map: &mut HashMap<String, Vec<String>>, module: String, files: Vec<String>) {
    if files.is_empty() {
        return;
    }
    let entry = map.entry(module).or_default();
    for fp in files {
        if !entry.contains(&fp) {
            entry.push(fp);
        }
    }
}

fn collect_files_for_prefix(prefix: &str, files_set: &HashSet<String>) -> Vec<String> {
    let normalized = prefix.trim_end_matches('/').to_string() + "/";
    let mut files = Vec::new();
    for fp in files_set.iter() {
        if fp.starts_with(&normalized) {
            files.push(fp.clone());
        }
    }
    files
}

fn find_xcode_project_files(project_root: &str) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let root = Path::new(project_root);
    if root
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext == "xcodeproj")
        .unwrap_or(false)
    {
        let pbx = root.join("project.pbxproj");
        if pbx.exists() {
            out.push(pbx);
        }
        return out;
    }

    let Ok(entries) = std::fs::read_dir(root) else {
        return out;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        if path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext == "xcodeproj")
            .unwrap_or(false)
        {
            let pbx = path.join("project.pbxproj");
            if pbx.exists() {
                out.push(pbx);
            }
            continue;
        }

        let Ok(subentries) = std::fs::read_dir(&path) else {
            continue;
        };
        for subentry in subentries.flatten() {
            let subpath = subentry.path();
            if !subpath.is_dir() {
                continue;
            }
            if subpath
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext == "xcodeproj")
                .unwrap_or(false)
            {
                let pbx = subpath.join("project.pbxproj");
                if pbx.exists() {
                    out.push(pbx);
                }
            }
        }
    }
    out
}

fn build_swift_module_map_from_xcode(project_root: &str, files_set: &HashSet<String>) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    let pbx_paths = find_xcode_project_files(project_root);
    if pbx_paths.is_empty() {
        return map;
    }

    for pbx in pbx_paths {
        let Ok(contents) = std::fs::read_to_string(&pbx) else {
            continue;
        };

        let mut in_root_group = false;
        let mut in_native_target = false;
        let mut current_id = String::new();
        let mut current_block = String::new();
        let mut depth = 0i32;
        let mut root_groups: HashMap<String, String> = HashMap::new();
        let mut target_groups: HashMap<String, Vec<String>> = HashMap::new();

        for line in contents.lines() {
            if line.contains("Begin PBXFileSystemSynchronizedRootGroup section") {
                in_root_group = true;
                continue;
            }
            if line.contains("End PBXFileSystemSynchronizedRootGroup section") {
                in_root_group = false;
                continue;
            }
            if line.contains("Begin PBXNativeTarget section") {
                in_native_target = true;
                continue;
            }
            if line.contains("End PBXNativeTarget section") {
                in_native_target = false;
                continue;
            }

            if !(in_root_group || in_native_target) {
                continue;
            }

            if depth == 0 && line.contains("= {") {
                current_id = line.split_whitespace().next().unwrap_or("").to_string();
                current_block.clear();
            }

            if !current_id.is_empty() {
                current_block.push_str(line);
                current_block.push('\n');
                for ch in line.chars() {
                    if ch == '{' {
                        depth += 1;
                    } else if ch == '}' {
                        depth -= 1;
                    }
                }

                if depth == 0 {
                    if in_root_group {
                        if let Some(path) = extract_pbx_value(&current_block, "path") {
                            root_groups.insert(current_id.clone(), normalize_pbx_path(&path));
                        }
                    } else if in_native_target {
                        let name = extract_pbx_value(&current_block, "name");
                        let groups = extract_pbx_id_array(&current_block, "fileSystemSynchronizedGroups");
                        if let (Some(name), false) = (name, groups.is_empty()) {
                            target_groups.insert(name, groups);
                        }
                    }
                    current_id.clear();
                    current_block.clear();
                }
            }
        }

        for (target, group_ids) in target_groups {
            let mut files = Vec::new();
            for group_id in group_ids {
                if let Some(path) = root_groups.get(&group_id) {
                    files.extend(collect_files_for_prefix(path, files_set));
                }
            }
            insert_module_files(&mut map, target, files);
        }
    }

    map
}

fn build_swift_module_map(project_root: &str, files_set: &HashSet<String>) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    let pkg_path = Path::new(project_root).join("Package.swift");
    let Ok(contents) = std::fs::read_to_string(&pkg_path) else {
        return build_swift_module_map_from_xcode(project_root, files_set);
    };

    let mut current = String::new();
    let mut depth = 0i32;
    let mut in_target = false;
    for line in contents.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(".target(")
            || trimmed.starts_with(".executableTarget(")
            || trimmed.starts_with(".testTarget(")
        {
            in_target = true;
            depth = 0;
            current.clear();
        }
        if in_target {
            for ch in trimmed.chars() {
                if ch == '(' {
                    depth += 1;
                } else if ch == ')' {
                    depth -= 1;
                }
            }
            current.push_str(trimmed);
            current.push('\n');
            if depth <= 0 {
                in_target = false;
                let name = extract_string_value(&current, "name:");
                let path = extract_string_value(&current, "path:");
                let sources = extract_string_array(&current, "sources:");
                let Some(name) = name else {
                    continue;
                };
                let base_path = path.unwrap_or_else(|| format!("Sources/{name}"));
                let mut files: Vec<String> = Vec::new();
                if !sources.is_empty() {
                    for src in sources {
                        let joined = format!("{}/{}", base_path.trim_end_matches('/'), src);
                        if files_set.contains(&joined) {
                            files.push(joined);
                        }
                    }
                } else {
                    let prefix = base_path.trim_end_matches('/').to_string() + "/";
                    for fp in files_set.iter() {
                        if fp.starts_with(&prefix) {
                            files.push(fp.clone());
                        }
                    }
                }
                insert_module_files(&mut map, name, files);
            }
        }
    }

    let xcode_map = build_swift_module_map_from_xcode(project_root, files_set);
    for (name, files) in xcode_map {
        insert_module_files(&mut map, name, files);
    }

    map
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
    } else if src_fp.ends_with(".rs") {
        let mut mod_str = module.to_string();
        if let Some(idx) = mod_str.rfind("use ") {
            mod_str = mod_str[idx + 4..].to_string();
        }
        mod_str = mod_str.trim().trim_end_matches(';').to_string();
        if let Some((before, _)) = mod_str.split_once(" as ") {
            mod_str = before.trim().to_string();
        }
        if let Some((before, _)) = mod_str.split_once('{') {
            mod_str = before.trim().trim_end_matches("::").to_string();
        }
        mod_str = mod_str.trim_end_matches("::").to_string();
        if mod_str.is_empty() {
            return None;
        }

        let mut module_dir = if src_fp.ends_with("/mod.rs") {
            src_fp.trim_end_matches("/mod.rs").to_string()
        } else {
            src_fp.trim_end_matches(".rs").to_string()
        };

        let mut crate_root: Option<String> = None;
        if let Some(idx) = src_fp.rfind("/src/") {
            crate_root = Some(src_fp[..idx + 4].to_string());
        } else if src_fp.starts_with("src/") {
            crate_root = Some("src".to_string());
        }

        let mut tail = mod_str.as_str();
        let mut super_count = 0usize;
        while tail.starts_with("super::") {
            super_count += 1;
            tail = &tail[7..];
        }

        let base_dir = if mod_str.starts_with("crate::") {
            tail = &mod_str[7..];
            crate_root?
        } else if tail.starts_with("self::") {
            tail = &tail[6..];
            module_dir.clone()
        } else if super_count > 0 {
            for _ in 0..super_count {
                if let Some(idx) = module_dir.rfind('/') {
                    module_dir.truncate(idx);
                } else {
                    return None;
                }
            }
            module_dir.clone()
        } else {
            return None;
        };

        let tail = tail.trim_matches(':').trim();
        let mut base_path = base_dir.trim_end_matches('/').to_string();
        if !tail.is_empty() {
            base_path.push('/');
            base_path.push_str(&tail.replace("::", "/"));
        }

        let mut candidates: Vec<String> = Vec::new();
        if tail.is_empty() {
            candidates.push(format!("{base_path}/lib.rs"));
            candidates.push(format!("{base_path}/main.rs"));
        }
        candidates.push(format!("{base_path}.rs"));
        candidates.push(format!("{base_path}/mod.rs"));

        for candidate in candidates {
            if files_set.contains(&candidate) {
                return Some(candidate);
            }
        }
        return None;
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

fn resolve_launch_path(src_fp: &str, raw: &str, project_root: &str, files_set: &HashSet<String>) -> Option<String> {
    let mut candidate = raw.trim().replace('\\', "/");
    if candidate.is_empty() {
        return None;
    }
    if candidate.starts_with("./") {
        candidate = candidate[2..].to_string();
    }
    if candidate.starts_with("/") {
        if let Some(stripped) = candidate.strip_prefix(project_root) {
            candidate = stripped.trim_start_matches('/').to_string();
        } else {
            return None;
        }
    }
    if candidate.starts_with("../") {
        let mut base = std::path::PathBuf::from(src_fp);
        base.pop();
        base.push(candidate);
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
        candidate = parts.join("/");
    }
    if candidate.ends_with(".py") && files_set.contains(&candidate) {
        return Some(candidate);
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

fn stable_hash_hex(input: &str) -> String {
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;
    let mut h = FNV_OFFSET;
    for b in input.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(FNV_PRIME);
    }
    format!("{:016x}", h)
}

fn tokenize_normalized(source: &[u8]) -> Vec<u64> {
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;

    let mut tokens = Vec::new();
    let mut i = 0;
    while i < source.len() {
        let b = source[i];
        if (b as char).is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if (b as char).is_ascii_alphabetic() || b == b'_' {
            let mut j = i + 1;
            while j < source.len() {
                let c = source[j];
                if (c as char).is_ascii_alphanumeric() || c == b'_' {
                    j += 1;
                } else {
                    break;
                }
            }
            let mut h = FNV_OFFSET;
            for ch in b"<id>" {
                h ^= *ch as u64;
                h = h.wrapping_mul(FNV_PRIME);
            }
            tokens.push(h);
            i = j;
            continue;
        }
        if (b as char).is_ascii_digit() {
            let mut j = i + 1;
            while j < source.len() {
                let c = source[j];
                if (c as char).is_ascii_digit() {
                    j += 1;
                } else {
                    break;
                }
            }
            let mut h = FNV_OFFSET;
            for ch in b"<num>" {
                h ^= *ch as u64;
                h = h.wrapping_mul(FNV_PRIME);
            }
            tokens.push(h);
            i = j;
            continue;
        }

        let punct = match b {
            b'{' | b'}' | b'(' | b')' | b'[' | b']' | b';' | b',' | b'.' | b':' | b'+' | b'-' | b'*' | b'/' | b'%'
            | b'<' | b'>' | b'=' => Some(b),
            _ => None,
        };
        if let Some(p) = punct {
            let mut h = FNV_OFFSET;
            h ^= p as u64;
            h = h.wrapping_mul(FNV_PRIME);
            tokens.push(h);
            i += 1;
            continue;
        }

        i += 1;
    }
    tokens
}

fn winnow_fingerprints(tokens: &[u64], k: usize, window: usize) -> HashSet<u64> {
    if tokens.len() < k {
        return HashSet::new();
    }
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;
    let mut hashes = Vec::new();
    for i in 0..=tokens.len() - k {
        let mut h = FNV_OFFSET;
        for t in &tokens[i..i + k] {
            h ^= *t;
            h = h.wrapping_mul(FNV_PRIME);
        }
        hashes.push(h);
    }
    if hashes.is_empty() {
        return HashSet::new();
    }
    if hashes.len() <= window {
        return [*hashes.iter().min().unwrap()].into_iter().collect();
    }
    let mut fps = HashSet::new();
    for i in 0..=hashes.len() - window {
        let mut min = hashes[i];
        for j in i..i + window {
            if hashes[j] < min {
                min = hashes[j];
            }
        }
        fps.insert(min);
    }
    fps
}

fn kgram_hashes(tokens: &[u64], k: usize) -> HashSet<u64> {
    if tokens.len() < k {
        return HashSet::new();
    }
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;
    let mut out = HashSet::new();
    for i in 0..=tokens.len() - k {
        let mut h = FNV_OFFSET;
        for t in &tokens[i..i + k] {
            h ^= *t;
            h = h.wrapping_mul(FNV_PRIME);
        }
        out.insert(h);
    }
    out
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

struct PythonInferredCallRow {
    caller_id: String,
    callee: String,
    callee_filepath: String,
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

struct CloneGroupRow {
    id: String,
    project_id: String,
    size: usize,
    method: String,
    score_min: f64,
    score_max: f64,
    score_avg: f64,
}

struct CloneMemberRow {
    gid: String,
    sid: String,
}

struct CloneCanonRow {
    gid: String,
    sid: String,
}

struct FileCloneGroupRow {
    id: String,
    project_id: String,
    size: usize,
    method: String,
    score_min: f64,
    score_max: f64,
    score_avg: f64,
}

struct FileCloneMemberRow {
    gid: String,
    filepath: String,
    project_id: String,
}

struct FileCloneCanonRow {
    gid: String,
    filepath: String,
    project_id: String,
}

struct LaunchEdgeRow {
    src_filepath: String,
    tgt_filepath: String,
    project_id: String,
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

struct ImplicitImportSymbolEdgeRow {
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

struct PythonFileContext {
    file_id: String,
    filepath: String,
    symbol_spans: Vec<(usize, usize, String)>,
    call_sites: Vec<tags::CallSite>,
    module_aliases: std::collections::HashMap<String, String>,
}

struct CloneCandidate {
    symbol_id: String,
    filepath: String,
    span_len: u32,
    token_set: HashSet<u64>,
    fingerprints: Vec<HashSet<u64>>,
    kgrams: HashSet<u64>,
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

async fn write_python_inferred_calls(graph: &Arc<Graph>, batch: &[PythonInferredCallRow]) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (caller:Node {id: item.caller_id}) \
         MATCH (callee:Node {project_id: item.pid, name: item.callee, filepath: item.callee_fp}) \
         WHERE (callee:Function OR callee:Class OR callee:Struct OR callee:Method) \
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

async fn write_clone_groups(graph: &Arc<Graph>, batch: &[CloneGroupRow]) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MERGE (g:CloneGroup {id: item.id}) \
         SET g.project_id = item.project_id, \
             g.size = item.size, \
             g.method = item.method, \
             g.score_min = item.score_min, \
             g.score_max = item.score_max, \
             g.score_avg = item.score_avg, \
             g.created_at = timestamp()"
            .to_string(),
    )
    .param("batch", bolt);
    let _ = graph.run(q).await;
}

async fn write_clone_members(graph: &Arc<Graph>, batch: &[CloneMemberRow]) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (g:CloneGroup {id: item.gid}) \
         MATCH (s:Node {id: item.sid}) \
         MERGE (s)-[:MEMBER_OF_CLONE_GROUP]->(g)"
            .to_string(),
    )
    .param("batch", bolt);
    let _ = graph.run(q).await;
}

async fn write_clone_canon(graph: &Arc<Graph>, batch: &[CloneCanonRow]) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (g:CloneGroup {id: item.gid}) \
         MATCH (s:Node {id: item.sid}) \
         MERGE (g)-[:HAS_CANONICAL]->(s)"
            .to_string(),
    )
    .param("batch", bolt);
    let _ = graph.run(q).await;
}

async fn write_file_clone_groups(graph: &Arc<Graph>, batch: &[FileCloneGroupRow]) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MERGE (g:FileCloneGroup {id: item.id}) \
         SET g.project_id = item.project_id, \
             g.size = item.size, \
             g.method = item.method, \
             g.score_min = item.score_min, \
             g.score_max = item.score_max, \
             g.score_avg = item.score_avg, \
             g.created_at = timestamp()"
            .to_string(),
    )
    .param("batch", bolt);
    let _ = graph.run(q).await;
}

async fn write_file_clone_members(graph: &Arc<Graph>, batch: &[FileCloneMemberRow]) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (g:FileCloneGroup {id: item.gid}) \
         MATCH (f:File {project_id: item.project_id, filepath: item.filepath}) \
         MERGE (f)-[:MEMBER_OF_FILE_CLONE_GROUP]->(g)"
            .to_string(),
    )
    .param("batch", bolt);
    let _ = graph.run(q).await;
}

async fn write_file_clone_canon(graph: &Arc<Graph>, batch: &[FileCloneCanonRow]) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (g:FileCloneGroup {id: item.gid}) \
         MATCH (f:File {project_id: item.project_id, filepath: item.filepath}) \
         MERGE (g)-[:HAS_CANONICAL]->(f)"
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

async fn write_implicit_import_symbol_edges(graph: &Arc<Graph>, batch: &[ImplicitImportSymbolEdgeRow]) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (a:File {id: item.src}) \
         MATCH (b:Node {id: item.tgt}) \
         MERGE (a)-[:IMPLICIT_IMPORTS_SYMBOL]->(b)"
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

async fn write_launch_edges(graph: &Arc<Graph>, batch: &[LaunchEdgeRow]) {
    let bolt = rows_to_bolt(batch, |r| r.to_value());
    let q = Query::new(
        "UNWIND $batch AS item \
         MATCH (a:File {project_id: item.pid, filepath: item.src}) \
         MATCH (b:File {project_id: item.pid, filepath: item.tgt}) \
         MERGE (a)-[:LAUNCHES]->(b)"
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
    python_context: Option<PythonFileContext>,
    clone_candidates: Vec<CloneCandidate>,
    db_delegates: Vec<String>,
    external_urls: Vec<String>,
    import_symbol_requests: Vec<ImportSymbolRequest>,
    launch_calls: Vec<String>,
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

        let pid = Arc::clone(&project_id);
        let parse_entry = |entry: &ManifestEntry| {
            // Language detection
            let lang_name = match ts_pack::detect_language_from_extension(&entry.ext) {
                Some(lang) => lang,
                None => {
                    eprintln!(
                        "[ts-pack-index] detect_language_from_extension failed: {}",
                        entry.rel_path
                    );
                    return None;
                }
            };
            if !ts_pack::has_language(lang_name) {
                if let Err(err) = ts_pack::download(&[lang_name]) {
                    eprintln!("[ts-pack-index] download failed: {lang} ({err})", lang = lang_name);
                    return None;
                }
            }

            // Read source — skip oversized files
            let source = match std::fs::read_to_string(&entry.abs_path) {
                Ok(source) => source,
                Err(err) => {
                    eprintln!("[ts-pack-index] read failed: {} ({})", entry.rel_path, err);
                    return None;
                }
            };
            if source.len() > MAX_FILE_BYTES {
                eprintln!(
                    "[ts-pack-index] skipped oversized file: {} ({})",
                    entry.rel_path,
                    source.len()
                );
                return None;
            }

            let proc_config = ts_pack::ProcessConfig::new(lang_name).all();
            let result = match ts_pack::process(&source, &proc_config) {
                Ok(result) => result,
                Err(err) => {
                    eprintln!("[ts-pack-index] process failed: {} ({})", entry.rel_path, err);
                    return None;
                }
            };

            if entry.rel_path.contains("duplication_demo") {
                eprintln!(
                    "[ts-pack-index] debug structure: {} (structure={}, symbols={}, imports={})",
                    entry.rel_path,
                    result.structure.len(),
                    result.symbols.len(),
                    result.imports.len(),
                );
            }

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
            let mut python_context: Option<PythonFileContext> = None;
            let mut local_clone_candidates: Vec<CloneCandidate> = Vec::new();

            // Build exported-name set from structural result + tags visibility
            let mut exported_names: std::collections::HashSet<String> =
                result.exports.iter().map(|e| e.name.clone()).collect();
            let tags_result = ts_pack::parse_string(lang_name, source.as_bytes())
                .ok()
                .and_then(|tree| tags::run_tags(lang_name, &tree, source.as_bytes()));

            // --- Consume tags result: split into exported names + call sites ---
            let (tag_exported, raw_call_sites, db_delegates, external_calls, const_strings, launch_calls) =
                match tags_result {
                    Some(tr) => (
                        tr.exported_names,
                        tr.call_sites,
                        tr.db_delegates,
                        tr.external_calls,
                        tr.const_strings,
                        tr.launch_calls,
                    ),
                    None => (
                        std::collections::HashSet::new(),
                        Vec::new(),
                        std::collections::HashSet::new(),
                        Vec::new(),
                        std::collections::HashMap::new(),
                        Vec::new(),
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

            if std::env::var("LM_PROXY_CLONE_ENRICH")
                .ok()
                .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
                .unwrap_or(true)
            {
                if let Some(functions) = symbols.get("Function") {
                    let source_bytes = source.as_bytes();
                    for sym in functions {
                        let start = sym.start_byte.min(source_bytes.len());
                        let end = sym.end_byte.min(source_bytes.len());
                        if end <= start {
                            continue;
                        }
                        let tokens = tokenize_normalized(&source_bytes[start..end]);
                        if tokens.len() < WINNOW_MIN_TOKENS {
                            let kgrams = kgram_hashes(&tokens, WINNOW_SMALL_K);
                            if kgrams.is_empty() {
                                continue;
                            }
                            let token_set: HashSet<u64> = tokens.into_iter().collect();
                            let span_len = sym.end_line.saturating_sub(sym.start_line);
                            local_clone_candidates.push(CloneCandidate {
                                symbol_id: sym.id.clone(),
                                filepath: sym.filepath.clone(),
                                span_len,
                                token_set,
                                fingerprints: vec![HashSet::new(), HashSet::new(), HashSet::new()],
                                kgrams,
                            });
                            continue;
                        }
                        let mut fps_small = HashSet::new();
                        let mut fps_medium = HashSet::new();
                        let mut fps_large = HashSet::new();
                        let mut kgrams = HashSet::new();
                        if tokens.len() < WINNOW_SMALL_TOKEN_THRESHOLD {
                            kgrams = kgram_hashes(&tokens, WINNOW_SMALL_K);
                            fps_small = winnow_fingerprints(&tokens, WINNOW_SMALL_K, WINNOW_SMALL_W);
                            if fps_small.len() < WINNOW_MIN_FINGERPRINTS.saturating_sub(4) {
                                fps_small.clear();
                            }
                        } else {
                            fps_small = winnow_fingerprints(&tokens, WINNOW_SMALL_K, WINNOW_SMALL_W);
                            if fps_small.len() < WINNOW_MIN_FINGERPRINTS.saturating_sub(4) {
                                fps_small.clear();
                            }
                            fps_medium = winnow_fingerprints(&tokens, WINNOW_MEDIUM_K, WINNOW_MEDIUM_W);
                            if fps_medium.len() < WINNOW_MIN_FINGERPRINTS.saturating_sub(4) {
                                fps_medium.clear();
                            }
                            fps_large = winnow_fingerprints(&tokens, WINNOW_LARGE_K, WINNOW_LARGE_W);
                            if fps_large.len() < WINNOW_MIN_FINGERPRINTS.saturating_sub(4) {
                                fps_large.clear();
                            }
                            if fps_small.is_empty() && fps_medium.is_empty() && fps_large.is_empty() {
                                kgrams = kgram_hashes(&tokens, WINNOW_SMALL_K);
                            }
                        }
                        if fps_small.is_empty() && fps_medium.is_empty() && fps_large.is_empty() && kgrams.is_empty() {
                            continue;
                        }
                        let token_set: HashSet<u64> = tokens.into_iter().collect();
                        let span_len = sym.end_line.saturating_sub(sym.start_line);
                        local_clone_candidates.push(CloneCandidate {
                            symbol_id: sym.id.clone(),
                            filepath: sym.filepath.clone(),
                            span_len,
                            token_set,
                            fingerprints: vec![fps_small, fps_medium, fps_large],
                            kgrams,
                        });
                    }
                }
            }

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

            if lang_name == "python" {
                let mut module_aliases: std::collections::HashMap<String, String> = std::collections::HashMap::new();
                for imp in &result.imports {
                    if imp.alias.is_none() || !imp.items.is_empty() {
                        continue;
                    }
                    let Some(alias) = imp.alias.as_ref() else {
                        continue;
                    };
                    if alias.is_empty() || imp.source.is_empty() {
                        continue;
                    }
                    module_aliases.insert(alias.clone(), imp.source.clone());
                }
                if !call_sites.is_empty() && !module_aliases.is_empty() {
                    python_context = Some(PythonFileContext {
                        file_id: file_id.clone(),
                        filepath: rel_path.clone(),
                        symbol_spans: symbol_spans.clone(),
                        call_sites: call_sites.clone(),
                        module_aliases,
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

                if !imp.source.is_empty() {
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
                python_context,
                clone_candidates: local_clone_candidates,
                db_delegates: if is_backend { db_delegates } else { Vec::new() },
                external_urls,
                import_symbol_requests,
                launch_calls,
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
        let mut fp_counts: Vec<HashMap<u64, usize>> = vec![HashMap::new(); 3];
        for cand in &clone_candidates {
            for (scale_idx, fps) in cand.fingerprints.iter().enumerate() {
                for fp in fps {
                    *fp_counts[scale_idx].entry(*fp).or_insert(0) += 1;
                }
            }
        }

        let mut fp_index_selected: Vec<HashMap<u64, Vec<usize>>> = vec![HashMap::new(); 3];
        for (idx, cand) in clone_candidates.iter().enumerate() {
            for (scale_idx, fps) in cand.fingerprints.iter().enumerate() {
                if fps.is_empty() {
                    continue;
                }
                let mut filtered: Vec<u64> = if fps.len() <= WINNOW_FORCE_ALL_HASHES_MAX_FPS {
                    fps.iter().copied().collect()
                } else {
                    fps.iter()
                        .filter(|h| fp_counts[scale_idx].get(h).copied().unwrap_or(0) <= WINNOW_BUCKET_LIMIT)
                        .copied()
                        .collect()
                };
                if filtered.is_empty() && WINNOW_FALLBACK_HASHES > 0 {
                    let mut sorted: Vec<u64> = fps.iter().copied().collect();
                    sorted.sort();
                    filtered = sorted.into_iter().take(WINNOW_FALLBACK_HASHES).collect();
                }
                for fp in filtered {
                    fp_index_selected[scale_idx].entry(fp).or_default().push(idx);
                }
            }
        }

        let mut pair_infos: HashMap<(usize, usize), [usize; 3]> = HashMap::new();
        for (scale_idx, index) in fp_index_selected.iter().enumerate() {
            for ids in index.values() {
                if ids.len() < 2 {
                    continue;
                }
                for i in 0..ids.len() {
                    for j in (i + 1)..ids.len() {
                        let a = ids[i];
                        let b = ids[j];
                        let key = if a < b { (a, b) } else { (b, a) };
                        let entry = pair_infos.entry(key).or_insert([0usize; 3]);
                        entry[scale_idx] += 1;
                    }
                }
            }
        }

        let mut kgram_index: HashMap<u64, Vec<usize>> = HashMap::new();
        for (idx, cand) in clone_candidates.iter().enumerate() {
            if cand.kgrams.is_empty() {
                continue;
            }
            for gram in &cand.kgrams {
                kgram_index.entry(*gram).or_default().push(idx);
            }
        }
        let mut kgram_pairs: HashSet<(usize, usize)> = HashSet::new();
        for ids in kgram_index.values() {
            if ids.len() < 2 {
                continue;
            }
            for i in 0..ids.len() {
                for j in (i + 1)..ids.len() {
                    let a = ids[i];
                    let b = ids[j];
                    let key = if a < b { (a, b) } else { (b, a) };
                    kgram_pairs.insert(key);
                    pair_infos.entry(key).or_insert([0usize; 3]);
                }
            }
        }

        let mut parent: Vec<usize> = (0..clone_candidates.len()).collect();
        let find = |parent: &mut Vec<usize>, x: usize| -> usize {
            let mut x = x;
            while parent[x] != x {
                parent[x] = parent[parent[x]];
                x = parent[x];
            }
            x
        };
        let union = |parent: &mut Vec<usize>, a: usize, b: usize| {
            let ra = find(parent, a);
            let rb = find(parent, b);
            if ra != rb {
                parent[rb] = ra;
            }
        };

        for ((a, b), shared_counts) in pair_infos {
            let cand_a = &clone_candidates[a];
            let cand_b = &clone_candidates[b];
            let mut max_overlap = 0.0;
            for scale_idx in 0..3 {
                let fa = &cand_a.fingerprints[scale_idx];
                let fb = &cand_b.fingerprints[scale_idx];
                let min_den = fa.len().min(fb.len());
                if min_den == 0 {
                    continue;
                }
                let shared = shared_counts[scale_idx];
                if shared == 0 {
                    continue;
                }
                let overlap = shared as f64 / min_den as f64;
                if overlap > max_overlap {
                    max_overlap = overlap;
                }
            }

            let ta = &cand_a.token_set;
            let tb = &cand_b.token_set;
            let token_jaccard = if ta.is_empty() || tb.is_empty() {
                0.0
            } else {
                let inter = ta.intersection(tb).count();
                let uni = ta.union(tb).count();
                inter as f64 / uni as f64
            };

            let kgram_jaccard = if kgram_pairs.contains(&(a, b)) {
                let ka = &cand_a.kgrams;
                let kb = &cand_b.kgrams;
                if ka.is_empty() || kb.is_empty() {
                    0.0
                } else {
                    let inter = ka.intersection(kb).count();
                    let uni = ka.union(kb).count();
                    inter as f64 / uni as f64
                }
            } else {
                0.0
            };

            if max_overlap < WINNOW_MIN_OVERLAP
                && token_jaccard < WINNOW_TOKEN_SIM_THRESHOLD
                && kgram_jaccard < WINNOW_KGRAM_SIM_THRESHOLD
            {
                continue;
            }
            let score = max_overlap.max(token_jaccard).max(kgram_jaccard);
            if score >= WINNOW_MIN_SCORE {
                union(&mut parent, a, b);
            }
        }

        let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
        for i in 0..clone_candidates.len() {
            let root = find(&mut parent, i);
            groups.entry(root).or_default().push(i);
        }

        let mut clone_group_rows = Vec::new();
        let mut clone_member_rows = Vec::new();
        let mut clone_canon_rows = Vec::new();
        let mut file_group_map: HashMap<String, Vec<String>> = HashMap::new();

        for members in groups.values() {
            if members.len() < 2 {
                continue;
            }
            let mut ids: Vec<String> = members
                .iter()
                .map(|idx| clone_candidates[*idx].symbol_id.clone())
                .collect();
            ids.sort();
            let gid = stable_hash_hex(&ids.join("|"));
            let mut canon = members[0];
            for idx in members {
                let cand = &clone_candidates[*idx];
                let canon_cand = &clone_candidates[canon];
                if cand.span_len > canon_cand.span_len
                    || (cand.span_len == canon_cand.span_len && cand.symbol_id < canon_cand.symbol_id)
                {
                    canon = *idx;
                }
            }
            clone_group_rows.push(CloneGroupRow {
                id: gid.clone(),
                project_id: project_id.to_string(),
                size: members.len(),
                method: "winnow+tokens".to_string(),
                score_min: WINNOW_MIN_SCORE,
                score_max: 1.0,
                score_avg: WINNOW_MIN_SCORE,
            });
            for idx in members {
                let cand = &clone_candidates[*idx];
                clone_member_rows.push(CloneMemberRow {
                    gid: gid.clone(),
                    sid: cand.symbol_id.clone(),
                });
                file_group_map
                    .entry(cand.filepath.clone())
                    .or_default()
                    .push(gid.clone());
            }
            clone_canon_rows.push(CloneCanonRow {
                gid: gid.clone(),
                sid: clone_candidates[canon].symbol_id.clone(),
            });
        }

        let mut file_group_rows = Vec::new();
        let mut file_member_rows = Vec::new();
        let mut file_canon_rows = Vec::new();
        let mut file_groups: HashMap<String, Vec<String>> = HashMap::new();
        for (fp, gids) in &mut file_group_map {
            gids.sort();
            gids.dedup();
            if gids.is_empty() {
                continue;
            }
            let fgid = stable_hash_hex(&gids.join("|"));
            file_groups.entry(fgid).or_default().push(fp.clone());
        }
        for (fgid, files) in file_groups {
            if files.len() < 2 {
                continue;
            }
            let mut files_sorted = files.clone();
            files_sorted.sort();
            file_group_rows.push(FileCloneGroupRow {
                id: fgid.clone(),
                project_id: project_id.to_string(),
                size: files_sorted.len(),
                method: "function-overlap".to_string(),
                score_min: WINNOW_MIN_SCORE,
                score_max: 1.0,
                score_avg: WINNOW_MIN_SCORE,
            });
            let canon = files_sorted[0].clone();
            for fp in &files_sorted {
                file_member_rows.push(FileCloneMemberRow {
                    gid: fgid.clone(),
                    filepath: fp.clone(),
                    project_id: project_id.to_string(),
                });
            }
            file_canon_rows.push(FileCloneCanonRow {
                gid: fgid.clone(),
                filepath: canon,
                project_id: project_id.to_string(),
            });
        }

        // Clear existing clone groups for this project
        let _ = graph
            .run(
                Query::new("MATCH (g:CloneGroup {project_id:$pid}) DETACH DELETE g".to_string())
                    .param("pid", project_id.to_string()),
            )
            .await;
        let _ = graph
            .run(
                Query::new("MATCH (g:FileCloneGroup {project_id:$pid}) DETACH DELETE g".to_string())
                    .param("pid", project_id.to_string()),
            )
            .await;

        if !clone_group_rows.is_empty() {
            stream::iter(clone_group_rows.chunks(REL_BATCH_SIZE))
                .for_each_concurrent(REL_CONCURRENCY, |chunk| {
                    let g = Arc::clone(&graph);
                    async move { write_clone_groups(&g, chunk).await }
                })
                .await;
            stream::iter(clone_member_rows.chunks(REL_BATCH_SIZE))
                .for_each_concurrent(REL_CONCURRENCY, |chunk| {
                    let g = Arc::clone(&graph);
                    async move { write_clone_members(&g, chunk).await }
                })
                .await;
            stream::iter(clone_canon_rows.chunks(REL_BATCH_SIZE))
                .for_each_concurrent(REL_CONCURRENCY, |chunk| {
                    let g = Arc::clone(&graph);
                    async move { write_clone_canon(&g, chunk).await }
                })
                .await;
        }

        if !file_group_rows.is_empty() {
            stream::iter(file_group_rows.chunks(REL_BATCH_SIZE))
                .for_each_concurrent(REL_CONCURRENCY, |chunk| {
                    let g = Arc::clone(&graph);
                    async move { write_file_clone_groups(&g, chunk).await }
                })
                .await;
            stream::iter(file_member_rows.chunks(REL_BATCH_SIZE))
                .for_each_concurrent(REL_CONCURRENCY, |chunk| {
                    let g = Arc::clone(&graph);
                    async move { write_file_clone_members(&g, chunk).await }
                })
                .await;
            stream::iter(file_canon_rows.chunks(REL_BATCH_SIZE))
                .for_each_concurrent(REL_CONCURRENCY, |chunk| {
                    let g = Arc::clone(&graph);
                    async move { write_file_clone_canon(&g, chunk).await }
                })
                .await;
        }
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
