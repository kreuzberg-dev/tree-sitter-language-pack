use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use crate::ManifestEntry;

pub(crate) fn external_api_id(project_id: &str, url: &str) -> String {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    url.hash(&mut hasher);
    format!("{}:external:{:x}", project_id, hasher.finish())
}

pub(crate) fn join_url(base: &str, path: &str) -> Option<String> {
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

pub(crate) fn clean_import_name(name: &str) -> String {
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

pub(crate) fn project_root_from_manifest(manifest: &[ManifestEntry]) -> Option<String> {
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
    for fp in files_set {
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

fn build_swift_module_map_from_xcode(
    project_root: &str,
    files_set: &HashSet<String>,
) -> HashMap<String, Vec<String>> {
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
                        let groups =
                            extract_pbx_id_array(&current_block, "fileSystemSynchronizedGroups");
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

pub(crate) fn build_swift_module_map(
    project_root: &str,
    files_set: &HashSet<String>,
) -> HashMap<String, Vec<String>> {
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
                    for fp in files_set {
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

pub(crate) fn resolve_module_path(
    src_fp: &str,
    module: &str,
    files_set: &HashSet<String>,
) -> Option<String> {
    let module = module.trim();
    if module.is_empty() {
        return None;
    }

    let base = if module.starts_with("./") || module.starts_with("../") {
        let mut base = PathBuf::from(src_fp);
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
            let mut base = PathBuf::from(src_fp);
            base.pop();
            for _ in 1..dot_count {
                base.pop();
            }
            if !mod_str.is_empty() {
                base.push(mod_str.replace('.', "/"));
            }
            base
        } else {
            PathBuf::from(mod_str.replace('.', "/"))
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

pub(crate) fn resolve_launch_path(
    src_fp: &str,
    raw: &str,
    project_root: &str,
    files_set: &HashSet<String>,
) -> Option<String> {
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
        let mut base = PathBuf::from(src_fp);
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
