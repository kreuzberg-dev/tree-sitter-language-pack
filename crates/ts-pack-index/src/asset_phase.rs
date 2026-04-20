use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::Arc;

use regex::Regex;
use tree_sitter_language_pack as ts_pack;

use crate::{
    ApiRouteCallRow, ApiRouteHandlerRow, CargoCrateFileRow, CargoCrateRow, CargoDependencyEdgeRow,
    CargoWorkspaceCrateRow, CargoWorkspaceRow, FileEdgeRow, FileNode, ResourceBackingRow, ResourceTargetEdgeRow,
    ResourceUsageRow, XcodeSchemeFileRow, XcodeSchemeRow, XcodeSchemeTargetRow, XcodeTargetFileRow, XcodeTargetRow,
    XcodeWorkspaceProjectRow, XcodeWorkspaceRow, graph_schema,
};

const HTTP_METHODS: &[&str] = &["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS", "ANY"];

pub(crate) struct AssetOutputs {
    pub(crate) asset_links: Vec<FileEdgeRow>,
    pub(crate) api_edges: Vec<FileEdgeRow>,
    pub(crate) api_route_calls: Vec<ApiRouteCallRow>,
    pub(crate) api_route_handlers: Vec<ApiRouteHandlerRow>,
    pub(crate) service_edges: Vec<FileEdgeRow>,
    pub(crate) resource_usages: Vec<ResourceUsageRow>,
    pub(crate) resource_backings: Vec<ResourceBackingRow>,
    pub(crate) xcode_targets: Vec<XcodeTargetRow>,
    pub(crate) xcode_target_files: Vec<XcodeTargetFileRow>,
    pub(crate) xcode_target_resources: Vec<ResourceTargetEdgeRow>,
    pub(crate) xcode_workspaces: Vec<XcodeWorkspaceRow>,
    pub(crate) xcode_workspace_projects: Vec<XcodeWorkspaceProjectRow>,
    pub(crate) xcode_schemes: Vec<XcodeSchemeRow>,
    pub(crate) xcode_scheme_targets: Vec<XcodeSchemeTargetRow>,
    pub(crate) xcode_scheme_files: Vec<XcodeSchemeFileRow>,
    pub(crate) cargo_crates: Vec<CargoCrateRow>,
    pub(crate) cargo_workspaces: Vec<CargoWorkspaceRow>,
    pub(crate) cargo_workspace_crates: Vec<CargoWorkspaceCrateRow>,
    pub(crate) cargo_crate_files: Vec<CargoCrateFileRow>,
    pub(crate) cargo_dependency_edges: Vec<CargoDependencyEdgeRow>,
}

pub(crate) fn prepare_asset_graph_facts(
    all_files: &[FileNode],
    file_facts: &HashMap<String, ts_pack::FileFacts>,
    manifest_abs: &HashMap<String, String>,
    project_id: &Arc<str>,
) -> AssetOutputs {
    let file_id_by_path: HashMap<String, String> =
        all_files.iter().map(|f| (f.filepath.clone(), f.id.clone())).collect();
    let file_paths: HashSet<String> = file_id_by_path.keys().cloned().collect();

    let asset_links = collect_html_asset_edges(&file_id_by_path, manifest_abs, project_id);
    let (api_edges, api_route_calls, api_route_handlers) =
        collect_api_edges(&file_id_by_path, file_facts, manifest_abs, project_id);
    let service_edges = collect_service_edges(&file_id_by_path, manifest_abs, project_id);
    let (
        resource_usages,
        resource_backings,
        xcode_targets,
        xcode_target_files,
        xcode_target_resources,
        xcode_workspaces,
        xcode_workspace_projects,
        xcode_schemes,
        xcode_scheme_targets,
        xcode_scheme_files,
    ) = collect_apple_graph_rows(&file_id_by_path, &file_paths, file_facts, manifest_abs, project_id);
    let (cargo_crates, cargo_workspaces, cargo_workspace_crates, cargo_crate_files, cargo_dependency_edges) =
        collect_cargo_graph_rows(&file_paths, file_facts, project_id);

    AssetOutputs {
        asset_links,
        api_edges,
        api_route_calls,
        api_route_handlers,
        service_edges,
        resource_usages,
        resource_backings,
        xcode_targets,
        xcode_target_files,
        xcode_target_resources,
        xcode_workspaces,
        xcode_workspace_projects,
        xcode_schemes,
        xcode_scheme_targets,
        xcode_scheme_files,
        cargo_crates,
        cargo_workspaces,
        cargo_workspace_crates,
        cargo_crate_files,
        cargo_dependency_edges,
    }
}

fn read_text(path: &str) -> String {
    match std::fs::metadata(path) {
        Ok(meta) if meta.len() > 1_000_000 => String::new(),
        Ok(_) => std::fs::read_to_string(path).unwrap_or_default(),
        Err(_) => String::new(),
    }
}

fn resolve_href(src_fp: &str, raw: &str, file_id_by_path: &HashMap<String, String>) -> Option<String> {
    let mut cleaned = raw
        .split('#')
        .next()
        .unwrap_or("")
        .split('?')
        .next()
        .unwrap_or("")
        .trim();
    if cleaned.is_empty()
        || cleaned.starts_with("http://")
        || cleaned.starts_with("https://")
        || cleaned.starts_with("//")
        || cleaned.starts_with("data:")
        || cleaned.starts_with("mailto:")
    {
        return None;
    }
    if cleaned.starts_with('/') {
        let candidate = cleaned.trim_start_matches('/');
        let public_candidate = if candidate.starts_with("assets/") {
            format!("src/public/{candidate}")
        } else {
            format!("src/public/{candidate}")
        };
        if file_id_by_path.contains_key(&public_candidate) {
            return Some(public_candidate);
        }
        cleaned = candidate;
        if file_id_by_path.contains_key(cleaned) {
            return Some(cleaned.to_string());
        }
        return None;
    }
    let src_dir = Path::new(src_fp).parent()?.to_string_lossy().replace('\\', "/");
    let joined = Path::new(&src_dir).join(cleaned);
    let normalized = joined.to_string_lossy().replace('\\', "/");
    if file_id_by_path.contains_key(&normalized) {
        Some(normalized)
    } else {
        None
    }
}

fn collect_html_asset_edges(
    file_id_by_path: &HashMap<String, String>,
    manifest_abs: &HashMap<String, String>,
    project_id: &Arc<str>,
) -> Vec<FileEdgeRow> {
    let script_re = Regex::new(r#"(?i)<script[^>]+src=["']([^"']+)["']"#).unwrap();
    let css_re = Regex::new(r#"(?i)<link[^>]+href=["']([^"']+)["']"#).unwrap();
    let mut seen = HashSet::new();
    let mut rows = Vec::new();
    for (fp, src_id) in file_id_by_path {
        if !fp.ends_with(".html") && !fp.ends_with(".astro") {
            continue;
        }
        let Some(abs) = manifest_abs.get(fp) else {
            continue;
        };
        let content = read_text(abs);
        if content.is_empty() {
            continue;
        }
        for caps in script_re.captures_iter(&content) {
            let Some(target) = resolve_href(fp, caps.get(1).map(|m| m.as_str()).unwrap_or_default(), file_id_by_path)
            else {
                continue;
            };
            if let Some(tgt_id) = file_id_by_path.get(&target) {
                if tgt_id != src_id && seen.insert((src_id.clone(), tgt_id.clone())) {
                    rows.push(FileEdgeRow {
                        src_filepath: fp.clone(),
                        tgt_filepath: target,
                        project_id: project_id.to_string(),
                    });
                }
            }
        }
        for caps in css_re.captures_iter(&content) {
            let href = caps.get(1).map(|m| m.as_str()).unwrap_or_default();
            if !href.to_ascii_lowercase().ends_with(".css") {
                continue;
            }
            let Some(target) = resolve_href(fp, href, file_id_by_path) else {
                continue;
            };
            if let Some(tgt_id) = file_id_by_path.get(&target) {
                if tgt_id != src_id && seen.insert((src_id.clone(), tgt_id.clone())) {
                    rows.push(FileEdgeRow {
                        src_filepath: fp.clone(),
                        tgt_filepath: target,
                        project_id: project_id.to_string(),
                    });
                }
            }
        }
    }
    rows
}

fn normalize_method(method: &str) -> String {
    let upper = method.trim().to_ascii_uppercase();
    if HTTP_METHODS.contains(&upper.as_str()) {
        upper
    } else {
        "ANY".to_string()
    }
}

fn route_path_from_file(fp: &str) -> Option<String> {
    let parts: Vec<&str> = fp.split('/').collect();
    if parts.len() < 2 {
        return None;
    }
    let mut idx = 0usize;
    while idx < parts.len() {
        if parts[idx] == "packages" || parts[idx] == "apps" {
            idx += 2;
        } else if parts[idx] == "src" {
            idx += 1;
        } else {
            break;
        }
    }
    let relevant = &parts[idx..];
    if relevant.is_empty() {
        return None;
    }
    if relevant[0] == "app" && Path::new(fp).file_name()?.to_string_lossy().starts_with("route.") {
        let route_parts = &relevant[1..relevant.len().saturating_sub(1)];
        return if route_parts.is_empty() {
            Some("/".to_string())
        } else {
            Some(format!("/{}", route_parts.join("/")))
        };
    }
    if relevant[0] == "pages" && relevant.get(1) == Some(&"api") {
        let mut rel: Vec<String> = relevant[2..].iter().map(|s| (*s).to_string()).collect();
        if rel.is_empty() {
            return Some("/api".to_string());
        }
        let stem = Path::new(rel.last()?).file_stem()?.to_string_lossy().to_string();
        if stem == "index" || stem == "route" {
            rel.pop();
        } else {
            rel.pop();
            rel.push(stem);
        }
        return if rel.is_empty() {
            Some("/api".to_string())
        } else {
            Some(format!("/api/{}", rel.join("/")))
        };
    }
    if relevant[0] == "api" {
        let mut rel: Vec<String> = relevant[1..].iter().map(|s| (*s).to_string()).collect();
        if rel.is_empty() {
            return Some("/api".to_string());
        }
        let stem = Path::new(rel.last()?).file_stem()?.to_string_lossy().to_string();
        if stem == "index" || stem == "route" {
            rel.pop();
        } else {
            rel.pop();
            rel.push(stem);
        }
        return if rel.is_empty() {
            Some("/api".to_string())
        } else {
            Some(format!("/api/{}", rel.join("/")))
        };
    }
    None
}

fn route_regex_from_path(path: &str) -> Regex {
    let mut parts = Vec::new();
    for part in path.split('/').filter(|p| !p.is_empty()) {
        if part.starts_with(':') {
            parts.push("[^/]+".to_string());
        } else if part == "*" {
            parts.push(".+".to_string());
        } else {
            parts.push(regex::escape(part));
        }
    }
    Regex::new(&format!("^/{}/?$", parts.join("/"))).unwrap()
}

fn detect_api_prefixes(manifest_abs: &HashMap<String, String>) -> Vec<String> {
    let app_use_re = Regex::new(r#"app\.use\(\s*["']([^"']+)["']\s*,"#).unwrap();
    let mut prefixes = Vec::new();
    for (fp, abs) in manifest_abs {
        if !fp.ends_with("app.ts")
            && !fp.ends_with("app.js")
            && !fp.ends_with("server.ts")
            && !fp.ends_with("server.js")
        {
            continue;
        }
        let content = read_text(abs);
        for caps in app_use_re.captures_iter(&content) {
            let prefix = caps
                .get(1)
                .map(|m| m.as_str())
                .unwrap_or_default()
                .trim_end_matches('/');
            if prefix.contains("/api") && !prefixes.iter().any(|p| p == prefix) {
                prefixes.push(prefix.to_string());
            }
        }
    }
    if prefixes.is_empty() {
        prefixes.push("/api".to_string());
    }
    prefixes
}

fn collect_api_edges(
    file_id_by_path: &HashMap<String, String>,
    file_facts: &HashMap<String, ts_pack::FileFacts>,
    manifest_abs: &HashMap<String, String>,
    project_id: &Arc<str>,
) -> (Vec<FileEdgeRow>, Vec<ApiRouteCallRow>, Vec<ApiRouteHandlerRow>) {
    let api_target_paths: Vec<String> = file_id_by_path
        .keys()
        .filter(|fp| is_api_target_path(fp))
        .cloned()
        .collect();

    let mut route_targets: HashMap<(String, String), String> = HashMap::new();
    let mut express_routes: Vec<(String, String, String)> = Vec::new();
    for (fp, fid) in file_id_by_path {
        if let Some(facts) = file_facts.get(fp) {
            for route in &facts.route_defs {
                let method = normalize_method(&route.method);
                if route.path.starts_with('/') {
                    route_targets
                        .entry((route.path.clone(), method.clone()))
                        .or_insert(fid.clone());
                    if is_api_route_source(fp) {
                        express_routes.push((route.path.clone(), method, fid.clone()));
                    }
                }
            }
        }
        if Path::new(fp)
            .file_name()
            .map(|n| n.to_string_lossy().starts_with("route."))
            .unwrap_or(false)
        {
            if let Some(route_path) = route_path_from_file(fp) {
                route_targets
                    .entry((route_path, "ANY".to_string()))
                    .or_insert(fid.clone());
            }
        }
    }

    let api_prefixes = detect_api_prefixes(manifest_abs);
    let mut prefixed = Vec::new();
    for (path, method, fid) in &express_routes {
        prefixed.push((path.clone(), method.clone(), fid.clone()));
        for prefix in &api_prefixes {
            let combined = format!("{}/{}", prefix.trim_end_matches('/'), path.trim_start_matches('/'));
            prefixed.push((combined.replace("//", "/"), method.clone(), fid.clone()));
        }
    }
    let express_routes = prefixed;
    let express_patterns: Vec<(Regex, String, String)> = express_routes
        .iter()
        .map(|(path, method, fid)| (route_regex_from_path(path), method.clone(), fid.clone()))
        .collect();

    let mut seen_api_edges = HashSet::new();
    let mut seen_route_calls = HashSet::new();
    let mut seen_handlers = HashSet::new();
    let mut api_edges = Vec::new();
    let mut route_calls = Vec::new();
    let mut route_handlers = Vec::new();
    let api_target_set: HashSet<String> = api_target_paths
        .iter()
        .filter_map(|fp| file_id_by_path.get(fp))
        .cloned()
        .collect();

    for (fp, src_id) in file_id_by_path {
        let Some(facts) = file_facts.get(fp) else {
            continue;
        };
        for call in &facts.http_calls {
            if !call.path.starts_with('/') {
                continue;
            }
            let clean = call
                .path
                .split('?')
                .next()
                .unwrap_or(&call.path)
                .split('#')
                .next()
                .unwrap_or(&call.path);
            let method = normalize_method(&call.method);
            let mut matched_handlers: Vec<String> = Vec::new();
            for ((path, route_method), fid) in &route_targets {
                if path == clean && (route_method == "ANY" || *route_method == method || method == "ANY") {
                    matched_handlers.push(fid.clone());
                    if seen_route_calls.insert((fp.clone(), path.clone(), route_method.clone())) {
                        route_calls.push(ApiRouteCallRow {
                            src_filepath: fp.clone(),
                            path: path.clone(),
                            method: route_method.clone(),
                            project_id: project_id.to_string(),
                        });
                    }
                    if seen_handlers.insert((path.clone(), route_method.clone(), fid.clone())) {
                        route_handlers.push(ApiRouteHandlerRow {
                            path: path.clone(),
                            method: route_method.clone(),
                            tgt_filepath: find_filepath(fid, file_id_by_path).unwrap_or_default(),
                            project_id: project_id.to_string(),
                        });
                    }
                }
            }
            for (pattern, route_method, fid) in &express_patterns {
                if pattern.is_match(clean) && (route_method == "ANY" || route_method == &method || method == "ANY") {
                    matched_handlers.push(fid.clone());
                    if seen_route_calls.insert((fp.clone(), clean.to_string(), route_method.clone())) {
                        route_calls.push(ApiRouteCallRow {
                            src_filepath: fp.clone(),
                            path: clean.to_string(),
                            method: route_method.clone(),
                            project_id: project_id.to_string(),
                        });
                    }
                    if seen_handlers.insert((clean.to_string(), route_method.clone(), fid.clone())) {
                        route_handlers.push(ApiRouteHandlerRow {
                            path: clean.to_string(),
                            method: route_method.clone(),
                            tgt_filepath: find_filepath(fid, file_id_by_path).unwrap_or_default(),
                            project_id: project_id.to_string(),
                        });
                    }
                }
            }
            if matched_handlers.is_empty() && clean.starts_with("/api/") {
                for target_id in &api_target_set {
                    matched_handlers.push(target_id.clone());
                }
            }
            for fid in matched_handlers {
                if fid != *src_id {
                    let tgt_fp = find_filepath(&fid, file_id_by_path).unwrap_or_default();
                    if !tgt_fp.is_empty() && seen_api_edges.insert((fp.clone(), tgt_fp.clone())) {
                        api_edges.push(FileEdgeRow {
                            src_filepath: fp.clone(),
                            tgt_filepath: tgt_fp,
                            project_id: project_id.to_string(),
                        });
                    }
                }
            }
        }
    }

    (api_edges, route_calls, route_handlers)
}

fn is_api_route_source(fp: &str) -> bool {
    let normalized = fp.replace('\\', "/");
    if !(normalized.contains("/api/")
        || normalized.starts_with("api/")
        || normalized.contains("/pages/api/")
        || normalized.contains("/app/"))
    {
        return false;
    }
    normalized.ends_with(".ts")
        || normalized.ends_with(".js")
        || normalized.ends_with(".tsx")
        || normalized.ends_with(".jsx")
}

fn is_api_target_path(fp: &str) -> bool {
    let normalized = fp.replace('\\', "/");
    if normalized.ends_with("openapi.yaml") || normalized.ends_with("openapi.json") {
        return true;
    }
    is_api_route_source(&normalized)
}

fn find_filepath(file_id: &str, file_id_by_path: &HashMap<String, String>) -> Option<String> {
    file_id_by_path
        .iter()
        .find_map(|(fp, fid)| if fid == file_id { Some(fp.clone()) } else { None })
}

fn collect_service_edges(
    file_id_by_path: &HashMap<String, String>,
    manifest_abs: &HashMap<String, String>,
    project_id: &Arc<str>,
) -> Vec<FileEdgeRow> {
    let service_files: Vec<(String, String, String)> = file_id_by_path
        .iter()
        .filter_map(|(fp, _)| {
            if !(fp.contains("/services/") || fp.starts_with("services/")) {
                return None;
            }
            let stem = Path::new(fp).file_stem()?.to_string_lossy().to_string();
            if stem == "index" || stem == "types" || stem.is_empty() {
                return None;
            }
            Some((fp.clone(), stem, project_id.to_string()))
        })
        .collect();
    let mut rows = Vec::new();
    let mut seen = HashSet::new();
    for (fp, _) in file_id_by_path {
        let is_backend = (fp.contains("/api/")
            || fp.contains("/webhooks/")
            || fp.contains("/jobs/")
            || fp.starts_with("api/")
            || fp.starts_with("webhooks/")
            || fp.starts_with("jobs/"))
            && (fp.ends_with(".ts") || fp.ends_with(".js") || fp.ends_with(".swift"));
        if !is_backend {
            continue;
        }
        let Some(abs) = manifest_abs.get(fp) else {
            continue;
        };
        let content = read_text(abs);
        if content.is_empty() {
            continue;
        }
        for (svc_fp, stem, _) in &service_files {
            let re = Regex::new(&format!(r"\b{}\b", regex::escape(stem))).unwrap();
            if re.is_match(&content) && seen.insert((fp.clone(), svc_fp.clone())) {
                rows.push(FileEdgeRow {
                    src_filepath: fp.clone(),
                    tgt_filepath: svc_fp.clone(),
                    project_id: project_id.to_string(),
                });
            }
        }
    }
    rows
}

fn resource_rel_name(kind: &str) -> Option<&'static str> {
    match kind {
        "image" => Some("USES_ASSET"),
        "color" => Some("USES_COLOR_ASSET"),
        "nib" => Some("USES_XIB"),
        "storyboard" => Some("USES_STORYBOARD"),
        _ => None,
    }
}

fn discover_apple_resources(file_paths: &HashSet<String>) -> HashMap<(String, String), String> {
    let mut map = HashMap::new();
    for fp in file_paths {
        if fp.contains(".imageset/") && fp.ends_with("/Contents.json") {
            if let Some(name) = fp.split('/').rev().nth(1).and_then(|s| s.strip_suffix(".imageset")) {
                map.entry(("image".to_string(), name.to_string())).or_insert(fp.clone());
            }
        } else if fp.contains(".colorset/") && fp.ends_with("/Contents.json") {
            if let Some(name) = fp.split('/').rev().nth(1).and_then(|s| s.strip_suffix(".colorset")) {
                map.entry(("color".to_string(), name.to_string())).or_insert(fp.clone());
            }
        } else if fp.ends_with(".xib") {
            if let Some(stem) = Path::new(fp).file_stem().map(|s| s.to_string_lossy().to_string()) {
                map.entry(("nib".to_string(), stem)).or_insert(fp.clone());
            }
        } else if fp.ends_with(".storyboard") {
            if let Some(stem) = Path::new(fp).file_stem().map(|s| s.to_string_lossy().to_string()) {
                map.entry(("storyboard".to_string(), stem)).or_insert(fp.clone());
            }
        }
    }
    map
}

fn collect_apple_graph_rows(
    file_id_by_path: &HashMap<String, String>,
    file_paths: &HashSet<String>,
    file_facts: &HashMap<String, ts_pack::FileFacts>,
    manifest_abs: &HashMap<String, String>,
    project_id: &Arc<str>,
) -> (
    Vec<ResourceUsageRow>,
    Vec<ResourceBackingRow>,
    Vec<XcodeTargetRow>,
    Vec<XcodeTargetFileRow>,
    Vec<ResourceTargetEdgeRow>,
    Vec<XcodeWorkspaceRow>,
    Vec<XcodeWorkspaceProjectRow>,
    Vec<XcodeSchemeRow>,
    Vec<XcodeSchemeTargetRow>,
    Vec<XcodeSchemeFileRow>,
) {
    let debug_apple_facts = std::env::var("TS_PACK_DEBUG_APPLE_FACTS")
        .ok()
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(false);
    let resource_catalog = discover_apple_resources(file_paths);
    let mut resource_usages = Vec::new();
    let mut resource_backings = Vec::new();
    let mut seen_usage = HashSet::new();
    let mut seen_backing = HashSet::new();
    for (fp, facts) in file_facts {
        let Some(_src_id) = file_id_by_path.get(fp) else {
            continue;
        };
        for reference in &facts.resource_refs {
            let Some(rel_name) = resource_rel_name(&reference.kind) else {
                continue;
            };
            let backing = resource_catalog
                .get(&(reference.kind.clone(), reference.name.clone()))
                .cloned();
            if seen_usage.insert((
                fp.clone(),
                rel_name.to_string(),
                reference.name.clone(),
                reference.kind.clone(),
            )) {
                resource_usages.push(ResourceUsageRow {
                    src_filepath: fp.clone(),
                    rel_name: rel_name.to_string(),
                    name: reference.name.clone(),
                    kind: reference.kind.clone(),
                    filepath: backing.clone(),
                    project_id: project_id.to_string(),
                });
            }
            if let Some(backing_fp) = backing {
                if seen_backing.insert((reference.name.clone(), reference.kind.clone(), backing_fp.clone())) {
                    resource_backings.push(ResourceBackingRow {
                        name: reference.name.clone(),
                        kind: reference.kind.clone(),
                        filepath: backing_fp,
                        project_id: project_id.to_string(),
                    });
                }
            }
        }
    }

    let mut targets = HashMap::<String, XcodeTargetRow>::new();
    let mut target_file_rows = Vec::new();
    let mut target_resource_rows = Vec::new();
    let mut seen_target_files = HashSet::new();
    let mut seen_target_resources = HashSet::new();
    let mut target_project_files = HashMap::<String, String>::new();
    let mut raw_memberships = Vec::<(String, String)>::new();

    for facts in file_facts.values() {
        for target in &facts.apple_targets {
            targets.entry(target.target_id.clone()).or_insert(XcodeTargetRow {
                target_id: target.target_id.clone(),
                name: target.name.clone(),
                project_file: target.project_file.clone(),
                project_id: project_id.to_string(),
            });
            target_project_files.insert(target.target_id.clone(), target.project_file.clone());
        }
        for bundled in &facts.apple_bundled_files {
            raw_memberships.push((bundled.target_id.clone(), bundled.filepath.clone()));
        }
        for synced in &facts.apple_synced_groups {
            raw_memberships.push((synced.target_id.clone(), synced.group_path.clone()));
        }
    }

    let mut resource_by_path = HashMap::<String, (String, String)>::new();
    for ((kind, name), path) in &resource_catalog {
        resource_by_path.insert(path.clone(), (name.clone(), kind.clone()));
    }

    for (target_id, raw_path) in raw_memberships {
        let project_file = target_project_files.get(&target_id).cloned().unwrap_or_default();
        let project_dir = apple_project_source_root(&project_file);
        let candidates = vec![
            raw_path.trim_start_matches("./").trim_matches('"').to_string(),
            if !project_dir.is_empty() {
                format!(
                    "{}/{}",
                    project_dir,
                    raw_path.trim_start_matches("./").trim_matches('"')
                )
                .replace("//", "/")
            } else {
                String::new()
            },
        ];
        for candidate in candidates.into_iter().filter(|c| !c.is_empty()) {
            if file_id_by_path.contains_key(&candidate) {
                if seen_target_files.insert((target_id.clone(), candidate.clone())) {
                    target_file_rows.push(XcodeTargetFileRow {
                        target_id: target_id.clone(),
                        filepath: candidate.clone(),
                        project_id: project_id.to_string(),
                    });
                }
            }
            if let Some((name, kind)) = resource_by_path.get(&candidate) {
                if seen_target_resources.insert((target_id.clone(), name.clone(), kind.clone())) {
                    target_resource_rows.push(ResourceTargetEdgeRow {
                        target_id: target_id.clone(),
                        name: name.clone(),
                        kind: kind.clone(),
                        filepath: Some(candidate.clone()),
                        project_id: project_id.to_string(),
                    });
                }
            }
            let prefix = format!("{}/", candidate.trim_end_matches('/'));
            for fp in file_paths {
                if !fp.starts_with(&prefix) {
                    continue;
                }
                if fp.contains(".xcassets/")
                    || fp.ends_with(".xib")
                    || fp.ends_with(".storyboard")
                    || fp.ends_with(".plist")
                {
                    if seen_target_files.insert((target_id.clone(), fp.clone())) {
                        target_file_rows.push(XcodeTargetFileRow {
                            target_id: target_id.clone(),
                            filepath: fp.clone(),
                            project_id: project_id.to_string(),
                        });
                    }
                    if let Some((name, kind)) = resource_by_path.get(fp) {
                        if seen_target_resources.insert((target_id.clone(), name.clone(), kind.clone())) {
                            target_resource_rows.push(ResourceTargetEdgeRow {
                                target_id: target_id.clone(),
                                name: name.clone(),
                                kind: kind.clone(),
                                filepath: Some(fp.clone()),
                                project_id: project_id.to_string(),
                            });
                        }
                    }
                }
            }
        }
    }

    let mut workspace_rows = HashMap::<String, XcodeWorkspaceRow>::new();
    let mut workspace_project_rows = Vec::new();
    let mut seen_workspace_projects = HashSet::new();
    for (workspace_path, facts) in file_facts {
        for item in &facts.apple_workspace_projects {
            workspace_rows
                .entry(workspace_path.clone())
                .or_insert(XcodeWorkspaceRow {
                    workspace_path: workspace_path.clone(),
                    name: workspace_display_name(workspace_path),
                    project_id: project_id.to_string(),
                });
            if file_id_by_path.contains_key(&item.project_file)
                && seen_workspace_projects.insert((workspace_path.clone(), item.project_file.clone()))
            {
                workspace_project_rows.push(XcodeWorkspaceProjectRow {
                    workspace_path: workspace_path.clone(),
                    filepath: item.project_file.clone(),
                    project_id: project_id.to_string(),
                });
            }
        }
    }

    let mut scheme_rows = HashMap::<String, XcodeSchemeRow>::new();
    let mut scheme_target_rows = Vec::new();
    let mut scheme_file_rows = Vec::new();
    let mut seen_scheme_targets = HashSet::new();
    let mut seen_scheme_files = HashSet::new();
    for (scheme_path, facts) in file_facts {
        for item in &facts.apple_scheme_targets {
            scheme_rows.entry(scheme_path.clone()).or_insert(XcodeSchemeRow {
                scheme_path: scheme_path.clone(),
                name: item.scheme_name.clone(),
                container_path: item.container_path.clone(),
                project_id: project_id.to_string(),
            });
            if targets.contains_key(&item.target_id)
                && seen_scheme_targets.insert((scheme_path.clone(), item.target_id.clone()))
            {
                scheme_target_rows.push(XcodeSchemeTargetRow {
                    scheme_path: scheme_path.clone(),
                    target_id: item.target_id.clone(),
                    project_id: project_id.to_string(),
                });
            }
            if file_id_by_path.contains_key(scheme_path) && seen_scheme_files.insert(scheme_path.clone()) {
                scheme_file_rows.push(XcodeSchemeFileRow {
                    scheme_path: scheme_path.clone(),
                    filepath: scheme_path.clone(),
                    project_id: project_id.to_string(),
                });
            }
        }
    }

    let blueprint_re = Regex::new(r#"BlueprintIdentifier\s*=\s*"([A-F0-9]{8,})""#).unwrap();
    let container_re = Regex::new(r#"ReferencedContainer\s*=\s*"([^"]+)""#).unwrap();
    for (scheme_path, abs_path) in manifest_abs {
        if !scheme_path.ends_with(".xcscheme") {
            continue;
        }
        let content = read_text(abs_path);
        if content.is_empty() {
            continue;
        }
        let name = Path::new(scheme_path)
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "Scheme".to_string());
        let container_path = container_re
            .captures(&content)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();
        scheme_rows.entry(scheme_path.clone()).or_insert(XcodeSchemeRow {
            scheme_path: scheme_path.clone(),
            name,
            container_path,
            project_id: project_id.to_string(),
        });
        if file_id_by_path.contains_key(scheme_path) && seen_scheme_files.insert(scheme_path.clone()) {
            scheme_file_rows.push(XcodeSchemeFileRow {
                scheme_path: scheme_path.clone(),
                filepath: scheme_path.clone(),
                project_id: project_id.to_string(),
            });
        }
        for caps in blueprint_re.captures_iter(&content) {
            let target_id = caps.get(1).map(|m| m.as_str()).unwrap_or_default().to_string();
            if target_id.is_empty() || !targets.contains_key(&target_id) {
                continue;
            }
            if seen_scheme_targets.insert((scheme_path.clone(), target_id.clone())) {
                scheme_target_rows.push(XcodeSchemeTargetRow {
                    scheme_path: scheme_path.clone(),
                    target_id,
                    project_id: project_id.to_string(),
                });
            }
        }
    }

    if debug_apple_facts {
        let apple_fact_files = file_facts
            .values()
            .filter(|facts| {
                !facts.apple_targets.is_empty()
                    || !facts.apple_bundled_files.is_empty()
                    || !facts.apple_synced_groups.is_empty()
                    || !facts.apple_workspace_projects.is_empty()
                    || !facts.apple_scheme_targets.is_empty()
            })
            .count();
        eprintln!(
            "[ts-pack-index] apple facts: fact_files={} targets={} target_files={} target_resources={} workspaces={} workspace_projects={} schemes={} scheme_targets={} scheme_files={}",
            apple_fact_files,
            targets.len(),
            target_file_rows.len(),
            target_resource_rows.len(),
            workspace_rows.len(),
            workspace_project_rows.len(),
            scheme_rows.len(),
            scheme_target_rows.len(),
            scheme_file_rows.len(),
        );
    }

    (
        resource_usages,
        resource_backings,
        targets.into_values().collect(),
        target_file_rows,
        target_resource_rows,
        workspace_rows.into_values().collect(),
        workspace_project_rows,
        scheme_rows.into_values().collect(),
        scheme_target_rows,
        scheme_file_rows,
    )
}

fn apple_project_source_root(project_file: &str) -> String {
    let normalized = project_file.replace('\\', "/");
    if let Some(bundle_root) = normalized.strip_suffix("/project.pbxproj") {
        return Path::new(bundle_root)
            .parent()
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .unwrap_or_default();
    }
    if normalized.ends_with(".xcodeproj") {
        return Path::new(&normalized)
            .parent()
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .unwrap_or_default();
    }
    Path::new(&normalized)
        .parent()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_default()
}

fn collect_cargo_graph_rows(
    file_paths: &HashSet<String>,
    file_facts: &HashMap<String, ts_pack::FileFacts>,
    project_id: &Arc<str>,
) -> (
    Vec<CargoCrateRow>,
    Vec<CargoWorkspaceRow>,
    Vec<CargoWorkspaceCrateRow>,
    Vec<CargoCrateFileRow>,
    Vec<CargoDependencyEdgeRow>,
) {
    let mut crate_rows = HashMap::<String, CargoCrateRow>::new();
    let mut workspace_rows = HashMap::<String, CargoWorkspaceRow>::new();
    let mut workspace_crate_rows = Vec::new();
    let mut crate_file_rows = Vec::new();
    let mut dependency_rows = Vec::new();
    let mut known_manifests = HashMap::<String, String>::new();
    let mut seen_workspace_crates = HashSet::new();
    let mut seen_crate_files = HashSet::new();
    let mut seen_dependencies = HashSet::new();

    for facts in file_facts.values() {
        for package in &facts.cargo_packages {
            known_manifests.insert(package.manifest_path.clone(), package.package_name.clone());
            crate_rows.entry(package.package_name.clone()).or_insert(CargoCrateRow {
                name: package.package_name.clone(),
                crate_name: package.crate_name.clone(),
                manifest_path: Some(package.manifest_path.clone()),
                project_id: project_id.to_string(),
            });
            if file_paths.contains(&package.manifest_path)
                && seen_crate_files.insert((package.package_name.clone(), package.manifest_path.clone()))
            {
                crate_file_rows.push(CargoCrateFileRow {
                    crate_name: package.package_name.clone(),
                    manifest_path: package.manifest_path.clone(),
                    project_id: project_id.to_string(),
                });
            }
        }
    }

    for facts in file_facts.values() {
        for package in &facts.cargo_packages {
            for dependency in facts
                .cargo_dependencies
                .iter()
                .filter(|item| item.manifest_path == package.manifest_path)
            {
                crate_rows
                    .entry(dependency.dependency_name.clone())
                    .or_insert(CargoCrateRow {
                        name: dependency.dependency_name.clone(),
                        crate_name: dependency.dependency_name.replace('-', "_"),
                        manifest_path: None,
                        project_id: project_id.to_string(),
                    });
                if seen_dependencies.insert((
                    package.package_name.clone(),
                    dependency.dependency_name.clone(),
                    dependency.section.clone(),
                )) {
                    dependency_rows.push(CargoDependencyEdgeRow {
                        src_crate_name: package.package_name.clone(),
                        tgt_crate_name: dependency.dependency_name.clone(),
                        section: dependency.section.clone(),
                        project_id: project_id.to_string(),
                    });
                }
            }
        }
    }

    for (manifest_path, facts) in file_facts {
        let workspace_members = &facts.cargo_workspace_members;
        if workspace_members.is_empty() {
            continue;
        }
        workspace_rows
            .entry(manifest_path.clone())
            .or_insert(CargoWorkspaceRow {
                manifest_path: manifest_path.clone(),
                name: cargo_workspace_display_name(manifest_path),
                project_id: project_id.to_string(),
            });
        for member in workspace_members {
            for member_manifest_path in
                resolve_cargo_workspace_member_paths(&member.member_manifest_path, known_manifests.keys())
            {
                if let Some(crate_name) = known_manifests.get(&member_manifest_path)
                    && seen_workspace_crates.insert((manifest_path.clone(), crate_name.clone()))
                {
                    workspace_crate_rows.push(CargoWorkspaceCrateRow {
                        workspace_manifest_path: manifest_path.clone(),
                        crate_name: crate_name.clone(),
                        project_id: project_id.to_string(),
                    });
                }
            }
        }
    }

    (
        crate_rows.into_values().collect(),
        workspace_rows.into_values().collect(),
        workspace_crate_rows,
        crate_file_rows,
        dependency_rows,
    )
}

fn resolve_cargo_workspace_member_paths<'a>(
    member_manifest_path: &str,
    known_manifests: impl Iterator<Item = &'a String>,
) -> Vec<String> {
    if !member_manifest_path.contains('*') {
        return vec![member_manifest_path.to_string()];
    }
    let pattern = format!(
        "^{}$",
        regex::escape(member_manifest_path)
            .replace("\\*", "[^/]+")
            .replace("\\?", "[^/]")
    );
    let Ok(re) = Regex::new(&pattern) else {
        return Vec::new();
    };
    known_manifests.filter(|path| re.is_match(path)).cloned().collect()
}

fn cargo_workspace_display_name(workspace_manifest_path: &str) -> String {
    Path::new(workspace_manifest_path)
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|s| s.to_str())
        .unwrap_or(graph_schema::NODE_LABEL_CARGO_WORKSPACE)
        .to_string()
}

fn workspace_display_name(workspace_path: &str) -> String {
    let path = Path::new(workspace_path);
    let parent = path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|s| s.to_str())
        .unwrap_or_default();
    let grandparent = path
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.file_name())
        .and_then(|s| s.to_str())
        .unwrap_or_default();
    if parent.ends_with(".xcworkspace") && grandparent.ends_with(".xcodeproj") {
        return grandparent.trim_end_matches(".xcodeproj").to_string();
    }
    parent.trim_end_matches(".xcworkspace").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter_language_pack::facts::{
        AppleBundledFileFact, AppleSchemeTargetFact, AppleTargetFact, AppleWorkspaceProjectFact,
    };
    use tree_sitter_language_pack::facts::{CargoDependencyFact, CargoPackageFact, CargoWorkspaceMemberFact};
    use tree_sitter_language_pack::facts::{HttpCallFact, RouteDefFact};

    #[test]
    fn collects_cargo_workspace_and_dependency_rows() {
        let mut file_facts = HashMap::new();
        file_facts.insert(
            "Cargo.toml".to_string(),
            ts_pack::FileFacts {
                cargo_workspace_members: vec![
                    CargoWorkspaceMemberFact {
                        workspace_manifest_path: "Cargo.toml".to_string(),
                        member_manifest_path: "crates/api/Cargo.toml".to_string(),
                    },
                    CargoWorkspaceMemberFact {
                        workspace_manifest_path: "Cargo.toml".to_string(),
                        member_manifest_path: "crates/*/Cargo.toml".to_string(),
                    },
                ],
                ..Default::default()
            },
        );
        file_facts.insert(
            "crates/api/Cargo.toml".to_string(),
            ts_pack::FileFacts {
                cargo_packages: vec![CargoPackageFact {
                    manifest_path: "crates/api/Cargo.toml".to_string(),
                    package_name: "api".to_string(),
                    crate_name: "api".to_string(),
                }],
                cargo_dependencies: vec![CargoDependencyFact {
                    manifest_path: "crates/api/Cargo.toml".to_string(),
                    dependency_name: "serde".to_string(),
                    section: "dependencies".to_string(),
                }],
                ..Default::default()
            },
        );
        file_facts.insert(
            "crates/core/Cargo.toml".to_string(),
            ts_pack::FileFacts {
                cargo_packages: vec![CargoPackageFact {
                    manifest_path: "crates/core/Cargo.toml".to_string(),
                    package_name: "core".to_string(),
                    crate_name: "core".to_string(),
                }],
                ..Default::default()
            },
        );

        let file_paths = HashSet::from([
            "Cargo.toml".to_string(),
            "crates/api/Cargo.toml".to_string(),
            "crates/core/Cargo.toml".to_string(),
        ]);
        let (crates, workspaces, workspace_crates, crate_files, dependencies) =
            collect_cargo_graph_rows(&file_paths, &file_facts, &Arc::from("proj"));

        assert!(
            crates
                .iter()
                .any(|row| row.name == "api" && row.manifest_path.as_deref() == Some("crates/api/Cargo.toml"))
        );
        assert!(
            crates
                .iter()
                .any(|row| row.name == "serde" && row.manifest_path.is_none())
        );
        assert!(workspaces.iter().any(|row| row.manifest_path == "Cargo.toml"));
        assert!(
            workspace_crates
                .iter()
                .any(|row| row.workspace_manifest_path == "Cargo.toml" && row.crate_name == "api")
        );
        assert!(
            workspace_crates
                .iter()
                .any(|row| row.workspace_manifest_path == "Cargo.toml" && row.crate_name == "core")
        );
        assert!(
            crate_files
                .iter()
                .any(|row| row.crate_name == "api" && row.manifest_path == "crates/api/Cargo.toml")
        );
        assert!(dependencies.iter().any(|row| {
            row.src_crate_name == "api" && row.tgt_crate_name == "serde" && row.section == "dependencies"
        }));
    }

    #[test]
    fn collects_xcode_rows_from_apple_facts() {
        let mut file_facts = HashMap::new();
        file_facts.insert(
            "FrameCreator.xcodeproj/project.pbxproj".to_string(),
            ts_pack::FileFacts {
                apple_targets: vec![
                    AppleTargetFact {
                        target_id: "AA000001".to_string(),
                        name: "FrameCreator".to_string(),
                        project_file: "FrameCreator.xcodeproj/project.pbxproj".to_string(),
                    },
                    AppleTargetFact {
                        target_id: "AA000002".to_string(),
                        name: "FrameCreatorWidgets".to_string(),
                        project_file: "FrameCreator.xcodeproj/project.pbxproj".to_string(),
                    },
                ],
                apple_bundled_files: vec![AppleBundledFileFact {
                    target_id: "AA000001".to_string(),
                    filepath: "FrameCreator/Assets.xcassets".to_string(),
                }],
                ..Default::default()
            },
        );
        file_facts.insert(
            "FrameCreator.xcodeproj/project.xcworkspace/contents.xcworkspacedata".to_string(),
            ts_pack::FileFacts {
                apple_workspace_projects: vec![AppleWorkspaceProjectFact {
                    workspace_path: "FrameCreator.xcodeproj/project.xcworkspace/contents.xcworkspacedata".to_string(),
                    project_file: "FrameCreator.xcodeproj/project.pbxproj".to_string(),
                }],
                ..Default::default()
            },
        );
        file_facts.insert(
            "FrameCreator.xcodeproj/xcshareddata/xcschemes/FrameCreator.xcscheme".to_string(),
            ts_pack::FileFacts {
                apple_scheme_targets: vec![
                    AppleSchemeTargetFact {
                        scheme_path: "FrameCreator.xcodeproj/xcshareddata/xcschemes/FrameCreator.xcscheme".to_string(),
                        scheme_name: "FrameCreator".to_string(),
                        container_path: "FrameCreator.xcodeproj/project.pbxproj".to_string(),
                        target_id: "AA000001".to_string(),
                    },
                    AppleSchemeTargetFact {
                        scheme_path: "FrameCreator.xcodeproj/xcshareddata/xcschemes/FrameCreator.xcscheme".to_string(),
                        scheme_name: "FrameCreator".to_string(),
                        container_path: "FrameCreator.xcodeproj/project.pbxproj".to_string(),
                        target_id: "AA000002".to_string(),
                    },
                ],
                ..Default::default()
            },
        );

        let file_paths = HashSet::from([
            "FrameCreator.xcodeproj/project.pbxproj".to_string(),
            "FrameCreator.xcodeproj/project.xcworkspace/contents.xcworkspacedata".to_string(),
            "FrameCreator.xcodeproj/xcshareddata/xcschemes/FrameCreator.xcscheme".to_string(),
            "FrameCreator/Assets.xcassets/AccentColor.colorset/Contents.json".to_string(),
            "FrameCreator/Assets.xcassets/PlaceholderFrame.imageset/Contents.json".to_string(),
        ]);
        let file_id_by_path = HashMap::from([
            (
                "FrameCreator.xcodeproj/project.pbxproj".to_string(),
                "file:pbxproj".to_string(),
            ),
            (
                "FrameCreator.xcodeproj/project.xcworkspace/contents.xcworkspacedata".to_string(),
                "file:workspace".to_string(),
            ),
            (
                "FrameCreator.xcodeproj/xcshareddata/xcschemes/FrameCreator.xcscheme".to_string(),
                "file:scheme".to_string(),
            ),
            (
                "FrameCreator/Assets.xcassets/AccentColor.colorset/Contents.json".to_string(),
                "file:accent".to_string(),
            ),
            (
                "FrameCreator/Assets.xcassets/PlaceholderFrame.imageset/Contents.json".to_string(),
                "file:placeholder".to_string(),
            ),
        ]);
        let manifest_abs = HashMap::from([
            (
                "FrameCreator.xcodeproj/xcshareddata/xcschemes/FrameCreator.xcscheme".to_string(),
                "/tmp/FrameCreator.xcodeproj/xcshareddata/xcschemes/FrameCreator.xcscheme".to_string(),
            ),
        ]);

        let (
            _,
            _,
            xcode_targets,
            xcode_target_files,
            xcode_target_resources,
            xcode_workspaces,
            xcode_workspace_projects,
            xcode_schemes,
            xcode_scheme_targets,
            xcode_scheme_files,
        ) =
            collect_apple_graph_rows(&file_id_by_path, &file_paths, &file_facts, &manifest_abs, &Arc::from("proj"));

        assert_eq!(xcode_targets.len(), 2);
        assert_eq!(xcode_target_files.len(), 2);
        assert_eq!(xcode_target_resources.len(), 2);
        assert_eq!(xcode_workspaces.len(), 1);
        assert_eq!(xcode_workspace_projects.len(), 1);
        assert_eq!(xcode_schemes.len(), 1);
        assert_eq!(xcode_scheme_targets.len(), 2);
        assert_eq!(xcode_scheme_files.len(), 1);
    }

    #[test]
    fn collects_multi_project_workspace_rows_from_apple_facts() {
        let mut file_facts = HashMap::new();
        file_facts.insert(
            "BGM.xcworkspace/contents.xcworkspacedata".to_string(),
            ts_pack::FileFacts {
                apple_workspace_projects: vec![
                    AppleWorkspaceProjectFact {
                        workspace_path: "BGM.xcworkspace/contents.xcworkspacedata".to_string(),
                        project_file: "BGMApp/BGMApp.xcodeproj/project.pbxproj".to_string(),
                    },
                    AppleWorkspaceProjectFact {
                        workspace_path: "BGM.xcworkspace/contents.xcworkspacedata".to_string(),
                        project_file: "BGMDriver/BGMDriver.xcodeproj/project.pbxproj".to_string(),
                    },
                ],
                ..Default::default()
            },
        );
        file_facts.insert(
            "BGMApp/BGMAppTests/NullAudio/AudioDriverExamples.xcodeproj/project.xcworkspace/contents.xcworkspacedata"
                .to_string(),
            ts_pack::FileFacts {
                apple_workspace_projects: vec![AppleWorkspaceProjectFact {
                    workspace_path:
                        "BGMApp/BGMAppTests/NullAudio/AudioDriverExamples.xcodeproj/project.xcworkspace/contents.xcworkspacedata"
                            .to_string(),
                    project_file:
                        "BGMApp/BGMAppTests/NullAudio/AudioDriverExamples.xcodeproj/project.pbxproj"
                            .to_string(),
                }],
                ..Default::default()
            },
        );

        let file_paths = HashSet::from([
            "BGM.xcworkspace/contents.xcworkspacedata".to_string(),
            "BGMApp/BGMApp.xcodeproj/project.pbxproj".to_string(),
            "BGMDriver/BGMDriver.xcodeproj/project.pbxproj".to_string(),
            "BGMApp/BGMAppTests/NullAudio/AudioDriverExamples.xcodeproj/project.xcworkspace/contents.xcworkspacedata"
                .to_string(),
            "BGMApp/BGMAppTests/NullAudio/AudioDriverExamples.xcodeproj/project.pbxproj".to_string(),
        ]);
        let file_id_by_path = HashMap::from([
            (
                "BGM.xcworkspace/contents.xcworkspacedata".to_string(),
                "file:bgm-workspace".to_string(),
            ),
            (
                "BGMApp/BGMApp.xcodeproj/project.pbxproj".to_string(),
                "file:bgm-app-project".to_string(),
            ),
            (
                "BGMDriver/BGMDriver.xcodeproj/project.pbxproj".to_string(),
                "file:bgm-driver-project".to_string(),
            ),
            (
                "BGMApp/BGMAppTests/NullAudio/AudioDriverExamples.xcodeproj/project.xcworkspace/contents.xcworkspacedata"
                    .to_string(),
                "file:nullaudio-workspace".to_string(),
            ),
            (
                "BGMApp/BGMAppTests/NullAudio/AudioDriverExamples.xcodeproj/project.pbxproj".to_string(),
                "file:nullaudio-project".to_string(),
            ),
        ]);

        let (
            _,
            _,
            _,
            _,
            _,
            xcode_workspaces,
            xcode_workspace_projects,
            _,
            _,
            _,
        ) = collect_apple_graph_rows(
            &file_id_by_path,
            &file_paths,
            &file_facts,
            &HashMap::new(),
            &Arc::from("proj"),
        );

        assert_eq!(xcode_workspaces.len(), 2);
        assert_eq!(xcode_workspace_projects.len(), 3);
        assert!(
            xcode_workspaces
                .iter()
                .any(|row| row.workspace_path == "BGM.xcworkspace/contents.xcworkspacedata")
        );
        assert!(
            xcode_workspaces.iter().any(|row| {
                row.workspace_path
                    == "BGMApp/BGMAppTests/NullAudio/AudioDriverExamples.xcodeproj/project.xcworkspace/contents.xcworkspacedata"
            })
        );
        assert!(
            xcode_workspace_projects.iter().any(|row| {
                row.workspace_path == "BGM.xcworkspace/contents.xcworkspacedata"
                    && row.filepath == "BGMApp/BGMApp.xcodeproj/project.pbxproj"
            })
        );
        assert!(
            xcode_workspace_projects.iter().any(|row| {
                row.workspace_path == "BGM.xcworkspace/contents.xcworkspacedata"
                    && row.filepath == "BGMDriver/BGMDriver.xcodeproj/project.pbxproj"
            })
        );
        assert!(
            xcode_workspace_projects.iter().any(|row| {
                row.workspace_path
                    == "BGMApp/BGMAppTests/NullAudio/AudioDriverExamples.xcodeproj/project.xcworkspace/contents.xcworkspacedata"
                    && row.filepath
                        == "BGMApp/BGMAppTests/NullAudio/AudioDriverExamples.xcodeproj/project.pbxproj"
            })
        );
    }

    #[test]
    fn collects_api_route_edges_from_named_route_files_under_api_routes() {
        let file_id_by_path = HashMap::from([
            ("src/public/assets/financial-summary.js".to_string(), "f1".to_string()),
            ("src/api/routes/financeAdminRoutes.ts".to_string(), "f2".to_string()),
            ("src/app.ts".to_string(), "f3".to_string()),
        ]);

        let file_facts = HashMap::from([
            (
                "src/public/assets/financial-summary.js".to_string(),
                ts_pack::FileFacts {
                    http_calls: vec![HttpCallFact {
                        client: "fetch".to_string(),
                        method: "GET".to_string(),
                        path: "/api/financials/tax-package".to_string(),
                    }],
                    ..Default::default()
                },
            ),
            (
                "src/api/routes/financeAdminRoutes.ts".to_string(),
                ts_pack::FileFacts {
                    route_defs: vec![RouteDefFact {
                        framework: "express".to_string(),
                        method: "GET".to_string(),
                        path: "/financials/tax-package".to_string(),
                    }],
                    ..Default::default()
                },
            ),
        ]);

        let manifest_abs = HashMap::from([("src/app.ts".to_string(), "/tmp/src/app.ts".to_string())]);

        std::fs::create_dir_all("/tmp/src").ok();
        std::fs::write("/tmp/src/app.ts", r#"app.use("/api", router);"#).unwrap();

        let (api_edges, route_calls, route_handlers) =
            collect_api_edges(&file_id_by_path, &file_facts, &manifest_abs, &Arc::from("proj"));

        assert!(api_edges.iter().any(|row| {
            row.src_filepath == "src/public/assets/financial-summary.js"
                && row.tgt_filepath == "src/api/routes/financeAdminRoutes.ts"
        }));
        assert!(route_calls.iter().any(|row| {
            row.src_filepath == "src/public/assets/financial-summary.js"
                && row.path == "/api/financials/tax-package"
                && row.method == "GET"
        }));
        assert!(route_handlers.iter().any(|row| {
            row.tgt_filepath == "src/api/routes/financeAdminRoutes.ts"
                && row.path == "/api/financials/tax-package"
                && row.method == "GET"
        }));
    }
}
