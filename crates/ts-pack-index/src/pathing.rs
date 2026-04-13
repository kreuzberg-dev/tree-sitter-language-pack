use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use crate::ManifestEntry;

pub(crate) fn external_api_id(project_id: &str, url: &str) -> String {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    url.hash(&mut hasher);
    format!("{}:external:{:x}", project_id, hasher.finish())
}

pub(crate) fn canonical_project_id(project_id: &str) -> &str {
    project_id
        .split_once("::shadow::")
        .map(|(canonical, _)| canonical)
        .unwrap_or(project_id)
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

fn is_swift_source_file(path: &str) -> bool {
    path.ends_with(".swift")
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
                    files.extend(
                        collect_files_for_prefix(path, files_set)
                            .into_iter()
                            .filter(|fp| is_swift_source_file(fp)),
                    );
                }
            }
            insert_module_files(&mut map, target, files);
        }
    }

    map
}

pub(crate) fn build_swift_module_map(project_root: &str, files_set: &HashSet<String>) -> HashMap<String, Vec<String>> {
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
                        if files_set.contains(&joined) && is_swift_source_file(&joined) {
                            files.push(joined);
                        }
                    }
                } else {
                    let prefix = base_path.trim_end_matches('/').to_string() + "/";
                    for fp in files_set {
                        if fp.starts_with(&prefix) && is_swift_source_file(fp) {
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

pub(crate) fn resolve_module_path(src_fp: &str, module: &str, files_set: &HashSet<String>) -> Option<String> {
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

    let mut candidate_bases = vec![base.clone()];
    for suffix in [".js", ".jsx", ".mjs", ".cjs"] {
        if let Some(stripped) = base.strip_suffix(suffix) {
            let stripped = stripped.to_string();
            if !candidate_bases.contains(&stripped) {
                candidate_bases.push(stripped);
            }
        }
    }

    for candidate_base in candidate_bases {
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
            let candidate = format!("{candidate_base}{suf}");
            if files_set.contains(&candidate) {
                return Some(candidate);
            }
        }
    }
    None
}

pub(crate) fn resolve_file_import_target(
    src_fp: &str,
    module: &str,
    files_set: &HashSet<String>,
    swift_module_map: &HashMap<String, Vec<String>>,
    stems: &HashMap<String, Vec<String>>,
) -> Option<String> {
    if let Some(target) = resolve_module_path(src_fp, module, files_set) {
        return Some(target);
    }

    if src_fp.ends_with(".swift") {
        if let Some(candidates) = swift_module_map.get(module)
            && let Some(first) = candidates.first()
        {
            return Some(first.clone());
        }
    }

    let module_tail = module
        .split(['.', '/'])
        .filter(|part| !part.is_empty())
        .next_back()
        .unwrap_or("");
    if module_tail.is_empty() {
        return None;
    }
    if let Some(candidates) = stems.get(module_tail)
        && candidates.len() == 1
    {
        return candidates.first().cloned();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_dir(name: &str) -> PathBuf {
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        std::env::temp_dir().join(format!("ts-pack-index-{name}-{nanos}"))
    }

    #[test]
    fn joins_urls_consistently() {
        assert_eq!(
            join_url("https://api.example.com", "/v1/test"),
            Some("https://api.example.com/v1/test".to_string())
        );
        assert_eq!(
            join_url("https://api.example.com/", "/v1/test"),
            Some("https://api.example.com/v1/test".to_string())
        );
        assert_eq!(join_url("not-a-url", "/v1/test"), None);
    }

    #[test]
    fn resolves_relative_ts_module_paths() {
        let files = HashSet::from([
            "src/main.ts".to_string(),
            "src/api/routes.ts".to_string(),
            "src/api/index.ts".to_string(),
        ]);

        assert_eq!(
            resolve_module_path("src/main.ts", "./api/routes", &files),
            Some("src/api/routes.ts".to_string())
        );
        assert_eq!(
            resolve_module_path("src/main.ts", "./api/index", &files),
            Some("src/api/index.ts".to_string())
        );
    }

    #[test]
    fn resolves_ts_source_imports_with_js_suffix_back_to_ts_files() {
        let files = HashSet::from([
            "packages/sdk/js/src/client.ts".to_string(),
            "packages/sdk/js/src/gen/client/types.gen.ts".to_string(),
        ]);

        assert_eq!(
            resolve_module_path("packages/sdk/js/src/client.ts", "./gen/client/types.gen.js", &files),
            Some("packages/sdk/js/src/gen/client/types.gen.ts".to_string())
        );
    }

    #[test]
    fn resolves_python_relative_module_paths() {
        let files = HashSet::from([
            "pkg/main.py".to_string(),
            "pkg/helpers.py".to_string(),
            "pkg/sub/__init__.py".to_string(),
        ]);

        assert_eq!(
            resolve_module_path("pkg/main.py", ".helpers", &files),
            Some("pkg/helpers.py".to_string())
        );
        assert_eq!(
            resolve_module_path("pkg/main.py", ".sub", &files),
            Some("pkg/sub/__init__.py".to_string())
        );
    }

    #[test]
    fn builds_swift_module_map_from_package_manifest() {
        let root = unique_temp_dir("package");
        fs::create_dir_all(root.join("Sources/App")).unwrap();
        fs::create_dir_all(root.join("Sources/Lib")).unwrap();
        fs::write(
            root.join("Package.swift"),
            r#"
            let package = Package(
              name: "Demo",
              targets: [
                .target(name: "App", path: "Sources/App"),
                .target(name: "Lib", path: "Sources/Lib")
              ]
            )
            "#,
        )
        .unwrap();

        let files = HashSet::from([
            "Sources/App/main.swift".to_string(),
            "Sources/Lib/util.swift".to_string(),
        ]);

        let map = build_swift_module_map(root.to_str().unwrap(), &files);
        assert_eq!(map.get("App"), Some(&vec!["Sources/App/main.swift".to_string()]));
        assert_eq!(map.get("Lib"), Some(&vec!["Sources/Lib/util.swift".to_string()]));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn excludes_non_swift_files_from_xcode_swift_module_map() {
        let root = unique_temp_dir("xcode-swift-only");
        fs::create_dir_all(root.join("FrameCreator.xcodeproj")).unwrap();
        fs::write(
            root.join("FrameCreator.xcodeproj/project.pbxproj"),
            r#"
/* Begin PBXFileSystemSynchronizedRootGroup section */
AA = {
    isa = PBXFileSystemSynchronizedRootGroup;
    path = FrameCreator;
};
/* End PBXFileSystemSynchronizedRootGroup section */

/* Begin PBXNativeTarget section */
BB = {
    isa = PBXNativeTarget;
    name = FrameCreator;
    fileSystemSynchronizedGroups = (
        AA,
    );
};
/* End PBXNativeTarget section */
            "#,
        )
        .unwrap();

        let files = HashSet::from([
            "FrameCreator/Views/ContentView.swift".to_string(),
            "FrameCreator/Assets.xcassets/Contents.json".to_string(),
            "FrameCreator/Assets.xcassets/AppIcon.appiconset/Contents.json".to_string(),
        ]);

        let map = build_swift_module_map(root.to_str().unwrap(), &files);
        assert_eq!(
            map.get("FrameCreator"),
            Some(&vec!["FrameCreator/Views/ContentView.swift".to_string()])
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn resolves_file_import_target_from_swift_module_map_or_unique_stem() {
        let files = HashSet::from([
            "FrameCreator/Views/SidebarView.swift".to_string(),
            "FrameCreator/ViewModels/EditorViewModel.swift".to_string(),
        ]);
        let mut swift_map = HashMap::new();
        swift_map.insert(
            "FrameCreator".to_string(),
            vec!["FrameCreator/Views/SidebarView.swift".to_string()],
        );
        let mut stems = HashMap::new();
        stems.insert(
            "EditorViewModel".to_string(),
            vec!["FrameCreator/ViewModels/EditorViewModel.swift".to_string()],
        );

        assert_eq!(
            resolve_file_import_target(
                "FrameCreator/Views/SidebarView.swift",
                "FrameCreator",
                &files,
                &swift_map,
                &stems,
            ),
            Some("FrameCreator/Views/SidebarView.swift".to_string())
        );
        assert_eq!(
            resolve_file_import_target(
                "FrameCreator/Views/SidebarView.swift",
                "EditorViewModel",
                &files,
                &swift_map,
                &stems,
            ),
            Some("FrameCreator/ViewModels/EditorViewModel.swift".to_string())
        );
    }
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
