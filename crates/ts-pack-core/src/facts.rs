use ahash::AHashMap;
use regex::Regex;
use std::path::Path;

use crate::Error;
use crate::extract::{CaptureOutput, ExtractionConfig, ExtractionPattern, ExtractionResult, MatchResult};

const HTTP_METHODS: &[&str] = &["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"];
const NON_HTTP_CLIENTS: &[&str] = &["router", "app", "server"];

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FileFacts {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    pub route_defs: Vec<RouteDefFact>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    pub http_calls: Vec<HttpCallFact>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    pub resource_refs: Vec<ResourceRefFact>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    pub apple_targets: Vec<AppleTargetFact>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    pub apple_bundled_files: Vec<AppleBundledFileFact>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    pub apple_synced_groups: Vec<AppleSyncedGroupFact>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    pub apple_workspace_projects: Vec<AppleWorkspaceProjectFact>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty", default))]
    pub apple_scheme_targets: Vec<AppleSchemeTargetFact>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RouteDefFact {
    pub framework: String,
    pub method: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HttpCallFact {
    pub client: String,
    pub method: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResourceRefFact {
    pub kind: String,
    pub name: String,
    pub callee: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AppleTargetFact {
    pub target_id: String,
    pub name: String,
    pub project_file: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AppleBundledFileFact {
    pub target_id: String,
    pub filepath: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AppleSyncedGroupFact {
    pub target_id: String,
    pub group_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AppleWorkspaceProjectFact {
    pub workspace_path: String,
    pub project_file: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AppleSchemeTargetFact {
    pub scheme_path: String,
    pub scheme_name: String,
    pub container_path: String,
    pub target_id: String,
}

pub fn extract_file_facts(source: &str, language: &str, file_path: Option<&str>) -> Result<FileFacts, Error> {
    let mut facts = FileFacts::default();
    if let Some(path) = file_path {
        parse_apple_file_facts(source, path, &mut facts);
    }
    let Some(config) = config_for_language(language) else {
        return Ok(finalize_file_facts(facts));
    };
    let raw = crate::extract_patterns(source, &config)?;
    Ok(parse_file_facts(&raw, language, file_path, facts))
}

fn parse_file_facts(raw: &ExtractionResult, language: &str, file_path: Option<&str>, mut facts: FileFacts) -> FileFacts {
    let lang = language.to_ascii_lowercase();

    if matches!(lang.as_str(), "typescript" | "tsx" | "javascript") {
        for m in pattern_matches(raw, "express_routes") {
            let caps = capture_texts(m);
            let method = normalize_method(first_capture(&caps, "method"));
            let path = first_capture(&caps, "path");
            if let (Some(method), Some(path)) = (method, path)
                && path.starts_with('/')
            {
                facts.route_defs.push(RouteDefFact {
                    framework: "express".to_string(),
                    method,
                    path: path.to_string(),
                });
            }
        }

        if let Some(inferred_path) = file_path.and_then(route_path_from_file) {
            for m in pattern_matches(raw, "route_methods") {
                let caps = capture_texts(m);
                if let Some(method) = normalize_method(first_capture(&caps, "method")) {
                    facts.route_defs.push(RouteDefFact {
                        framework: "file_route".to_string(),
                        method,
                        path: inferred_path.clone(),
                    });
                }
            }
        }

        let pending_methods: Vec<Option<String>> = pattern_matches(raw, "http_method_props")
            .iter()
            .map(|m| {
                let caps = capture_texts(m);
                normalize_method(first_capture(&caps, "method"))
            })
            .collect();

        for m in pattern_matches(raw, "http_member_calls") {
            let caps = capture_texts(m);
            let client = first_capture(&caps, "client");
            let method = normalize_method(first_capture(&caps, "method")).unwrap_or_else(|| "ANY".to_string());
            let path = first_capture(&caps, "path");
            if let (Some(client), Some(path)) = (client, path)
                && path.starts_with('/')
                && !NON_HTTP_CLIENTS.contains(&client)
            {
                facts.http_calls.push(HttpCallFact {
                    client: client.to_string(),
                    method,
                    path: path.to_string(),
                });
            }
        }

        for (idx, m) in pattern_matches(raw, "http_fetch_calls").iter().enumerate() {
            let caps = capture_texts(m);
            let client = first_capture(&caps, "client");
            let path = first_capture(&caps, "path");
            if let (Some(client), Some(path)) = (client, path)
                && path.starts_with('/')
            {
                facts.http_calls.push(HttpCallFact {
                    client: client.to_string(),
                    method: pending_methods
                        .get(idx)
                        .and_then(|v| v.clone())
                        .unwrap_or_else(|| "ANY".to_string()),
                    path: path.to_string(),
                });
            }
        }
    }

    if lang == "swift" {
        for m in pattern_matches(raw, "resource_calls") {
            let caps = capture_texts(m);
            let callee = first_capture(&caps, "callee");
            let name = first_capture(&caps, "name");
            let kind = match callee {
                Some("Image" | "UIImage" | "NSImage") => Some("image"),
                Some("Color") => Some("color"),
                Some("UINib" | "NSNib") => Some("nib"),
                _ => None,
            };
            if let (Some(kind), Some(callee), Some(name)) = (kind, callee, name) {
                facts.resource_refs.push(ResourceRefFact {
                    kind: kind.to_string(),
                    name: name.to_string(),
                    callee: callee.to_string(),
                });
            }
        }
    }

    finalize_file_facts(facts)
}

fn finalize_file_facts(mut facts: FileFacts) -> FileFacts {
    facts.route_defs.sort();
    facts.route_defs.dedup();
    facts.http_calls.sort();
    facts.http_calls.dedup();
    facts.resource_refs.sort();
    facts.resource_refs.dedup();
    facts.apple_targets.sort();
    facts.apple_targets.dedup();
    facts.apple_bundled_files.sort();
    facts.apple_bundled_files.dedup();
    facts.apple_synced_groups.sort();
    facts.apple_synced_groups.dedup();
    facts.apple_workspace_projects.sort();
    facts.apple_workspace_projects.dedup();
    facts.apple_scheme_targets.sort();
    facts.apple_scheme_targets.dedup();
    facts
}

fn parse_apple_file_facts(source: &str, file_path: &str, facts: &mut FileFacts) {
    let normalized = file_path.replace('\\', "/");
    if normalized.ends_with(".xcodeproj/project.pbxproj") {
        parse_pbxproj_facts(source, &normalized, facts);
    } else if normalized.ends_with(".xcworkspace/contents.xcworkspacedata") {
        parse_workspace_facts(source, &normalized, facts);
    } else if normalized.ends_with(".xcscheme") {
        parse_scheme_facts(source, &normalized, facts);
    }
}

fn parse_pbxproj_facts(source: &str, file_path: &str, facts: &mut FileFacts) {
    let project_file = file_path.trim_end_matches("/project.pbxproj").to_string();
    let target_re = Regex::new(
        r#"(?s)([A-F0-9]{8,}) /\* [^*]+ \*/ = \{\s*isa = PBXNativeTarget;.*?\bname = ([^;]+);"#,
    )
    .unwrap();
    for caps in target_re.captures_iter(source) {
        let target_id = caps.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
        let name = caps
            .get(2)
            .map(|m| m.as_str().trim().trim_matches('"').to_string())
            .unwrap_or_default();
        if !target_id.is_empty() && !name.is_empty() {
            facts.apple_targets.push(AppleTargetFact {
                target_id,
                name,
                project_file: project_file.clone(),
            });
        }
    }

    let build_file_re = Regex::new(
        r#"([A-F0-9]{8,}) /\* [^*]+ \*/ = \{\s*isa = PBXBuildFile;\s*fileRef = ([A-F0-9]{8,})"#,
    )
    .unwrap();
    let file_ref_re = Regex::new(
        r#"(?s)([A-F0-9]{8,}) /\* [^*]+ \*/ = \{\s*isa = PBXFileReference;.*?\bpath = ([^;]+);.*?\bsourceTree = ([^;]+);"#,
    )
    .unwrap();
    let resources_re = Regex::new(
        r#"(?s)([A-F0-9]{8,}) /\* Resources \*/ = \{\s*isa = PBXResourcesBuildPhase;.*?\bfiles = \((.*?)\);"#,
    )
    .unwrap();
    let target_phases_re = Regex::new(
        r#"(?s)([A-F0-9]{8,}) /\* [^*]+ \*/ = \{\s*isa = PBXNativeTarget;.*?\bbuildPhases = \((.*?)\);"#,
    )
    .unwrap();
    let synced_group_re = Regex::new(
        r#"(?s)([A-F0-9]{8,}) /\* [^*]+ \*/ = \{\s*isa = PBXFileSystemSynchronizedRootGroup;.*?\bpath = ([^;]+);"#,
    )
    .unwrap();
    let target_synced_re = Regex::new(
        r#"(?s)([A-F0-9]{8,}) /\* [^*]+ \*/ = \{\s*isa = PBXNativeTarget;.*?\bfileSystemSynchronizedGroups = \((.*?)\);"#,
    )
    .unwrap();
    let id_re = Regex::new(r#"([A-F0-9]{8,}) /\*"#).unwrap();

    let mut build_file_to_ref: AHashMap<String, String> = AHashMap::new();
    for caps in build_file_re.captures_iter(source) {
        build_file_to_ref.insert(caps[1].to_string(), caps[2].to_string());
    }

    let mut file_ref_to_path: AHashMap<String, String> = AHashMap::new();
    for caps in file_ref_re.captures_iter(source) {
        let clean_path = caps[2].trim().trim_matches('"');
        let source_tree = caps[3].trim().trim_matches('"');
        if !clean_path.is_empty() && source_tree != "BUILT_PRODUCTS_DIR" {
            file_ref_to_path.insert(caps[1].to_string(), clean_path.to_string());
        }
    }

    let mut phase_to_files: AHashMap<String, Vec<String>> = AHashMap::new();
    for caps in resources_re.captures_iter(source) {
        phase_to_files.insert(
            caps[1].to_string(),
            id_re
                .captures_iter(caps.get(2).map(|m| m.as_str()).unwrap_or_default())
                .map(|m| m[1].to_string())
                .collect(),
        );
    }

    let mut target_to_phases: AHashMap<String, Vec<String>> = AHashMap::new();
    for caps in target_phases_re.captures_iter(source) {
        target_to_phases.insert(
            caps[1].to_string(),
            id_re
                .captures_iter(caps.get(2).map(|m| m.as_str()).unwrap_or_default())
                .map(|m| m[1].to_string())
                .collect(),
        );
    }

    for (target_id, phase_ids) in target_to_phases {
        for phase_id in phase_ids {
            for build_file_id in phase_to_files.get(&phase_id).cloned().unwrap_or_default() {
                if let Some(file_ref_id) = build_file_to_ref.get(&build_file_id)
                    && let Some(path) = file_ref_to_path.get(file_ref_id)
                {
                    facts.apple_bundled_files.push(AppleBundledFileFact {
                        target_id: target_id.clone(),
                        filepath: normalize_pbxproj_relative_path(&project_file, path),
                    });
                }
            }
        }
    }

    let mut synced_group_paths: AHashMap<String, String> = AHashMap::new();
    for caps in synced_group_re.captures_iter(source) {
        let path = caps[2].trim().trim_matches('"');
        if !path.is_empty() {
            synced_group_paths.insert(caps[1].to_string(), path.to_string());
        }
    }
    for caps in target_synced_re.captures_iter(source) {
        let target_id = caps[1].to_string();
        for group_caps in id_re.captures_iter(caps.get(2).map(|m| m.as_str()).unwrap_or_default()) {
            if let Some(group_path) = synced_group_paths.get(&group_caps[1]) {
                facts.apple_synced_groups.push(AppleSyncedGroupFact {
                    target_id: target_id.clone(),
                    group_path: normalize_pbxproj_relative_path(&project_file, group_path),
                });
            }
        }
    }
}

fn parse_workspace_facts(source: &str, file_path: &str, facts: &mut FileFacts) {
    let workspace_dir = Path::new(file_path)
        .parent()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_default();
    let file_ref_re = Regex::new(r#"location\s*=\s*"([^"]+)""#).unwrap();
    for caps in file_ref_re.captures_iter(source) {
        let location = caps.get(1).map(|m| m.as_str()).unwrap_or_default().trim();
        if location == "self:" {
            let project_file = Path::new(&workspace_dir)
                .parent()
                .map(|p| format!("{}/project.pbxproj", p.to_string_lossy().replace('\\', "/")))
                .unwrap_or_default();
            if !project_file.is_empty() {
                facts.apple_workspace_projects.push(AppleWorkspaceProjectFact {
                    workspace_path: file_path.to_string(),
                    project_file,
                });
            }
            continue;
        }
        let rel_ref = location.split_once(':').map(|(_, rhs)| rhs).unwrap_or(location).trim();
        if !rel_ref.ends_with(".xcodeproj") {
            continue;
        }
        let project_file = normalize_workspace_project_path(file_path, rel_ref);
        facts.apple_workspace_projects.push(AppleWorkspaceProjectFact {
            workspace_path: file_path.to_string(),
            project_file,
        });
    }
}

fn parse_scheme_facts(source: &str, file_path: &str, facts: &mut FileFacts) {
    let scheme_name = Path::new(file_path)
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_string();
    let buildable_re = Regex::new(
        r#"BlueprintIdentifier\s*=\s*"([^"]+)".*?ReferencedContainer\s*=\s*"([^"]+)""#,
    )
    .unwrap();
    for caps in buildable_re.captures_iter(source) {
        let target_id = caps.get(1).map(|m| m.as_str().trim().to_string()).unwrap_or_default();
        let container = caps.get(2).map(|m| m.as_str().trim()).unwrap_or_default();
        let container_path = normalize_scheme_container_path(file_path, container);
        if !target_id.is_empty() {
            facts.apple_scheme_targets.push(AppleSchemeTargetFact {
                scheme_path: file_path.to_string(),
                scheme_name: scheme_name.clone(),
                container_path,
                target_id,
            });
        }
    }
}

fn normalize_pbxproj_relative_path(project_file: &str, raw_path: &str) -> String {
    let clean = raw_path.trim().trim_matches('"').trim_start_matches("./");
    if clean.is_empty() {
        return clean.to_string();
    }
    let project_dir = Path::new(project_file)
        .parent()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_default();
    if project_dir.is_empty() {
        clean.to_string()
    } else {
        format!("{project_dir}/{clean}")
    }
}

fn normalize_workspace_project_path(workspace_path: &str, rel_ref: &str) -> String {
    let clean = rel_ref.trim().trim_start_matches("./");
    if clean.ends_with(".xcodeproj") {
        return format!("{clean}/project.pbxproj");
    }
    let workspace_dir = Path::new(workspace_path)
        .parent()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_default();
    let candidate = if workspace_dir.is_empty() {
        clean.to_string()
    } else {
        format!("{workspace_dir}/{clean}")
    };
    if candidate.ends_with(".xcodeproj") {
        format!("{candidate}/project.pbxproj")
    } else {
        candidate
    }
}

fn normalize_scheme_container_path(scheme_path: &str, container_ref: &str) -> String {
    let rel_ref = container_ref
        .split_once(':')
        .map(|(_, rhs)| rhs)
        .unwrap_or(container_ref)
        .trim()
        .trim_start_matches("./");
    if rel_ref.ends_with(".xcodeproj") {
        return format!("{rel_ref}/project.pbxproj");
    }
    let scheme_dir = Path::new(scheme_path)
        .parent()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_default();
    let candidate = if scheme_dir.is_empty() {
        rel_ref.to_string()
    } else {
        format!("{scheme_dir}/{rel_ref}")
    };
    if candidate.ends_with(".xcodeproj") {
        format!("{candidate}/project.pbxproj")
    } else {
        candidate
    }
}

fn first_capture<'a>(caps: &'a AHashMap<String, Vec<String>>, name: &str) -> Option<&'a str> {
    caps.get(name).and_then(|values| values.first().map(String::as_str))
}

fn capture_texts(m: &MatchResult) -> AHashMap<String, Vec<String>> {
    let mut out = AHashMap::new();
    for cap in &m.captures {
        if let Some(text) = &cap.text {
            out.entry(cap.name.clone()).or_insert_with(Vec::new).push(text.clone());
        }
    }
    out
}

fn pattern_matches<'a>(raw: &'a ExtractionResult, name: &str) -> &'a [MatchResult] {
    raw.results
        .get(name)
        .map(|entry| entry.matches.as_slice())
        .unwrap_or(&[])
}

fn normalize_method(value: Option<&str>) -> Option<String> {
    let method = value?.trim().to_ascii_uppercase();
    if HTTP_METHODS.contains(&method.as_str()) {
        Some(method)
    } else {
        None
    }
}

fn text_pattern(query: &str, max_results: usize) -> ExtractionPattern {
    ExtractionPattern {
        query: query.to_string(),
        capture_output: CaptureOutput::Text,
        child_fields: Vec::new(),
        max_results: Some(max_results),
        byte_range: None,
    }
}

fn config_for_language(language: &str) -> Option<ExtractionConfig> {
    let normalized = language.to_ascii_lowercase();
    let patterns = match normalized.as_str() {
        "javascript" | "typescript" | "tsx" => web_patterns(),
        "swift" => swift_patterns(),
        _ => return None,
    };
    Some(ExtractionConfig {
        language: normalized,
        patterns,
    })
}

fn web_patterns() -> AHashMap<String, ExtractionPattern> {
    let mut patterns = AHashMap::new();
    patterns.insert(
        "express_routes".to_string(),
        text_pattern(
            "(call_expression \
               function: (member_expression \
                 object: (identifier) @router \
                 property: (property_identifier) @method) \
               arguments: (arguments (string (string_fragment) @path))) @route_call",
            200,
        ),
    );
    patterns.insert(
        "http_member_calls".to_string(),
        text_pattern(
            "[(call_expression \
                function: (member_expression object: (identifier) @client property: (property_identifier) @method) \
                arguments: (arguments (string (string_fragment) @path))) \
              (call_expression \
                function: (member_expression object: (call_expression function: (identifier) @client) property: (property_identifier) @method) \
                arguments: (arguments (string (string_fragment) @path)))] @http_call",
            200,
        ),
    );
    patterns.insert(
        "http_fetch_calls".to_string(),
        text_pattern(
            "(call_expression \
               function: (identifier) @client \
               arguments: (arguments (string (string_fragment) @path))) @http_call \
             (#eq? @client \"fetch\")",
            200,
        ),
    );
    patterns.insert(
        "http_method_props".to_string(),
        text_pattern(
            "(pair \
               key: (property_identifier) @key \
               value: (string (string_fragment) @method)) @method_pair \
             (#eq? @key \"method\")",
            200,
        ),
    );
    patterns.insert(
        "route_methods".to_string(),
        text_pattern(
            "[(function_declaration name: (identifier) @method) \
              (lexical_declaration (variable_declarator name: (identifier) @method))]",
            50,
        ),
    );
    patterns
}

fn swift_patterns() -> AHashMap<String, ExtractionPattern> {
    let mut patterns = AHashMap::new();
    patterns.insert(
        "resource_calls".to_string(),
        text_pattern(
            "[(call_expression \
                called_expression: (simple_identifier) @callee \
                arguments: (call_suffix (value_arguments (value_argument (string_literal (string_literal_content) @name))))) \
              (call_expression \
                called_expression: (member_access_expr name: (simple_identifier) @callee) \
                arguments: (call_suffix (value_arguments (value_argument (string_literal (string_literal_content) @name)))))] @resource_call",
            200,
        ),
    );
    patterns
}

fn route_path_from_file(file_path: &str) -> Option<String> {
    let normalized = file_path.replace('\\', "/");
    let path = Path::new(&normalized);
    let parts: Vec<String> = path
        .components()
        .filter_map(|component| {
            let value = component.as_os_str().to_str()?;
            if value.is_empty() {
                None
            } else {
                Some(value.to_string())
            }
        })
        .collect();
    if parts.len() < 2 {
        return None;
    }

    let file_name = path.file_name()?.to_str()?;
    let mut idx = 0usize;
    while idx < parts.len() && matches!(parts[idx].as_str(), "packages" | "apps" | "src") {
        if matches!(parts[idx].as_str(), "packages" | "apps") && idx + 1 < parts.len() {
            idx += 2;
        } else {
            idx += 1;
        }
    }
    let relevant = &parts[idx..];
    if relevant.is_empty() {
        return None;
    }

    if relevant[0] == "app" && file_name.starts_with("route.") {
        let route_parts = &relevant[1..relevant.len().saturating_sub(1)];
        return Some(if route_parts.is_empty() {
            "/".to_string()
        } else {
            format!("/{}", route_parts.join("/"))
        });
    }

    if relevant.len() > 1 && relevant[0] == "pages" && relevant[1] == "api" {
        return route_path_from_segments(&relevant[2..]);
    }

    if relevant[0] == "api" {
        return route_path_from_segments(&relevant[1..]);
    }

    None
}

fn route_path_from_segments(segments: &[String]) -> Option<String> {
    if segments.is_empty() {
        return Some("/api".to_string());
    }
    let mut rel = segments.to_vec();
    let stem = Path::new(rel.last()?).file_stem()?.to_str()?.to_string();
    if matches!(stem.as_str(), "index" | "route") {
        rel.pop();
    } else if let Some(last) = rel.last_mut() {
        *last = stem;
    }

    Some(if rel.is_empty() {
        "/api".to_string()
    } else {
        format!("/api/{}", rel.join("/"))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_typescript_route_and_http_facts() {
        if !crate::has_language("typescript") {
            return;
        }

        let source = r#"
            export async function GET() {}
            router.post("/api/leases");
            const data = await fetch("/api/units", { method: "POST" });
            await client.get("/api/properties");
        "#;

        let facts = extract_file_facts(source, "typescript", Some("src/api/leases/route.ts")).unwrap();
        assert!(
            facts
                .route_defs
                .iter()
                .any(|item| item.method == "POST" && item.path == "/api/leases")
        );
        assert!(
            facts
                .route_defs
                .iter()
                .any(|item| item.method == "GET" && item.path == "/api/leases")
        );
        assert!(!facts.http_calls.iter().any(|item| item.client == "router"));
        assert!(
            facts
                .http_calls
                .iter()
                .any(|item| item.client == "fetch" && item.method == "POST" && item.path == "/api/units")
        );
        assert!(
            facts
                .http_calls
                .iter()
                .any(|item| item.client == "client" && item.method == "GET" && item.path == "/api/properties")
        );
    }

    #[test]
    fn extracts_swift_resource_refs() {
        if !crate::has_language("swift") {
            return;
        }

        let source = r#"
            let image = Image("hero")
            let color = Color("brand")
            let nib = UINib(nibName: "MainView", bundle: nil)
        "#;

        let facts = extract_file_facts(source, "swift", None).unwrap();
        assert!(
            facts
                .resource_refs
                .iter()
                .any(|item| item.kind == "image" && item.name == "hero")
        );
        assert!(
            facts
                .resource_refs
                .iter()
                .any(|item| item.kind == "color" && item.name == "brand")
        );
        assert!(
            facts
                .resource_refs
                .iter()
                .any(|item| item.kind == "nib" && item.name == "MainView")
        );
    }

    #[test]
    fn infers_route_path_from_file_layout() {
        assert_eq!(
            route_path_from_file("src/pages/api/users/index.ts"),
            Some("/api/users".to_string())
        );
        assert_eq!(
            route_path_from_file("apps/web/src/app/projects/[id]/route.ts"),
            Some("/projects/[id]".to_string())
        );
    }

    #[test]
    fn extracts_xcode_project_facts() {
        let source = r#"
AA000001 /* App */ = {
    isa = PBXNativeTarget;
    buildPhases = (
        AA000010 /* Resources */,
    );
    fileSystemSynchronizedGroups = (
        AA000020 /* App */,
    );
    name = App;
};
AA000010 /* Resources */ = {
    isa = PBXResourcesBuildPhase;
    files = (
        AA000101 /* Main.storyboard in Resources */,
    );
};
AA000101 /* Main.storyboard in Resources */ = { isa = PBXBuildFile; fileRef = AA000201 /* Main.storyboard */; };
AA000201 /* Main.storyboard */ = { isa = PBXFileReference; path = "App/Main.storyboard"; sourceTree = "<group>"; };
AA000020 /* App */ = { isa = PBXFileSystemSynchronizedRootGroup; path = App; sourceTree = "<group>"; };
"#;
        let facts = extract_file_facts(source, "text", Some("ios/App.xcodeproj/project.pbxproj")).unwrap();
        assert!(facts.apple_targets.iter().any(|item| item.name == "App" && item.target_id == "AA000001"));
        assert!(facts.apple_bundled_files.iter().any(|item| item.filepath == "ios/App/Main.storyboard"));
        assert!(facts.apple_synced_groups.iter().any(|item| item.group_path == "ios/App"));
    }

    #[test]
    fn extracts_xcode_workspace_and_scheme_facts() {
        let workspace = r#"<Workspace version="1.0"><FileRef location="self:" /></Workspace>"#;
        let scheme = r#"
<Scheme>
  <BuildAction>
    <BuildActionEntries>
      <BuildActionEntry>
        <BuildableReference BlueprintIdentifier="AA000001" ReferencedContainer="container:App.xcodeproj" />
      </BuildActionEntry>
    </BuildActionEntries>
  </BuildAction>
</Scheme>
"#;
        let workspace_facts = extract_file_facts(
            workspace,
            "text",
            Some("ios/App.xcodeproj/project.xcworkspace/contents.xcworkspacedata"),
        )
        .unwrap();
        assert!(
            workspace_facts
                .apple_workspace_projects
                .iter()
                .any(|item| item.project_file == "ios/App.xcodeproj/project.pbxproj")
        );

        let scheme_facts = extract_file_facts(
            scheme,
            "text",
            Some("ios/App.xcodeproj/xcshareddata/xcschemes/App.xcscheme"),
        )
        .unwrap();
        assert!(
            scheme_facts
                .apple_scheme_targets
                .iter()
                .any(|item| item.target_id == "AA000001" && item.container_path == "App.xcodeproj/project.pbxproj")
        );
    }
}
