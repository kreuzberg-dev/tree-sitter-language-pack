use serde::Deserialize;
use std::collections::HashSet;
use std::path::Path;

/// A single E2E test fixture loaded from JSON.
#[derive(Debug, Clone, Deserialize)]
pub struct Fixture {
    pub id: String,
    pub category: String,
    pub description: String,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub source_code: Option<String>,
    #[serde(default)]
    pub assertions: Option<Assertions>,
    #[serde(default)]
    pub skip: Option<SkipConfig>,
    #[serde(default)]
    #[allow(dead_code)]
    pub tags: Vec<String>,
}

/// Assertions to verify in the generated test.
#[derive(Debug, Clone, Deserialize)]
pub struct Assertions {
    #[serde(default)]
    pub tree_not_null: Option<bool>,
    #[serde(default)]
    pub root_child_count_min: Option<usize>,
    #[serde(default)]
    pub root_contains_node_type: Option<String>,
    #[serde(default)]
    pub expect_error: Option<bool>,
    #[serde(default)]
    pub has_error_nodes: Option<bool>,
    #[serde(default)]
    pub language_available: Option<bool>,
    #[serde(default)]
    pub languages_not_empty: Option<bool>,
    // Process assertions (process / process with chunking)
    #[serde(default)]
    pub process_language: Option<String>,
    #[serde(default)]
    pub process_structure_count_min: Option<usize>,
    #[serde(default)]
    pub process_structure_contains_kind: Option<String>,
    #[serde(default)]
    pub process_imports_count_min: Option<usize>,
    #[serde(default)]
    pub process_metrics_total_lines_min: Option<usize>,
    #[serde(default)]
    pub process_metrics_error_count: Option<usize>,
    #[serde(default)]
    pub process_diagnostics_not_empty: Option<bool>,
    #[serde(default)]
    pub process_chunk_count_min: Option<usize>,
    #[serde(default)]
    pub process_chunk_max_size: Option<usize>,
    #[serde(default)]
    pub process_comments_count_min: Option<usize>,
    #[serde(default)]
    pub process_exports_count_min: Option<usize>,
    #[serde(default)]
    pub process_imports_contains_source: Option<String>,
    #[serde(default)]
    pub process_structure_name_contains: Option<String>,
    #[serde(default)]
    pub process_metrics_code_lines_min: Option<usize>,
    #[serde(default)]
    pub process_metrics_comment_lines_min: Option<usize>,
    #[serde(default)]
    pub process_metrics_max_depth_min: Option<usize>,
    // Language detection
    #[serde(default)]
    pub detect_from_extension: Option<String>,
    #[serde(default)]
    pub detect_from_path: Option<String>,
    #[serde(default)]
    pub detect_from_content: Option<String>,
    #[serde(default)]
    pub detect_result: Option<String>,
    #[serde(default)]
    pub detect_result_none: Option<bool>,
    // Ambiguity
    #[serde(default)]
    pub ambiguity_extension: Option<String>,
    #[serde(default)]
    pub ambiguity_assigned: Option<String>,
    #[serde(default)]
    pub ambiguity_alternatives_contain: Option<String>,
    #[serde(default)]
    pub ambiguity_is_none: Option<bool>,
    // Highlight queries
    #[serde(default)]
    pub highlights_query_not_empty: Option<bool>,
    #[serde(default)]
    pub highlights_query_is_none: Option<bool>,
}

/// Configuration for when a test should be skipped.
#[derive(Debug, Clone, Deserialize)]
pub struct SkipConfig {
    #[serde(default)]
    pub requires_language: Option<String>,
}

/// Load all fixture JSON files from a directory tree.
///
/// Walks the directory recursively, loads all `.json` files (skipping `schema.json`),
/// sorts by (category, id), and detects duplicate IDs.
pub fn load_fixtures(dir: &Path) -> Result<Vec<Fixture>, String> {
    let mut fixtures = Vec::new();
    walk_dir(dir, &mut fixtures)?;

    // Sort by (category, id)
    fixtures.sort_by(|a, b| a.category.cmp(&b.category).then_with(|| a.id.cmp(&b.id)));

    // Detect duplicates
    let mut seen = HashSet::new();
    for fixture in &fixtures {
        if !seen.insert(&fixture.id) {
            return Err(format!("Duplicate fixture id: {}", fixture.id));
        }
    }

    Ok(fixtures)
}

fn walk_dir(dir: &Path, fixtures: &mut Vec<Fixture>) -> Result<(), String> {
    let entries = std::fs::read_dir(dir).map_err(|e| format!("Failed to read directory {}: {}", dir.display(), e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read dir entry: {e}"))?;
        let path = entry.path();

        if path.is_dir() {
            walk_dir(&path, fixtures)?;
        } else if path.extension().is_some_and(|ext| ext == "json") {
            // Skip schema.json
            if path.file_name().is_some_and(|name| name == "schema.json") {
                continue;
            }

            let content =
                std::fs::read_to_string(&path).map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

            // Support both single-fixture objects and arrays of fixtures.
            let value: serde_json::Value =
                serde_json::from_str(&content).map_err(|e| format!("Failed to parse {}: {}", path.display(), e))?;

            if value.is_array() {
                let batch: Vec<Fixture> = serde_json::from_value(value)
                    .map_err(|e| format!("Failed to parse fixture array in {}: {}", path.display(), e))?;
                fixtures.extend(batch);
            } else {
                let fixture: Fixture = serde_json::from_value(value)
                    .map_err(|e| format!("Failed to parse fixture in {}: {}", path.display(), e))?;
                fixtures.push(fixture);
            }
        }
    }

    Ok(())
}

/// Sanitize a string for use as a function/test name.
/// Replaces spaces, hyphens, and other non-alphanumeric chars with underscores,
/// and converts to lowercase.
pub fn sanitize_name(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect()
}

/// Group fixtures by category, returning sorted (category, fixtures) pairs.
pub fn group_by_category(fixtures: &[Fixture]) -> Vec<(String, Vec<&Fixture>)> {
    let mut map: std::collections::BTreeMap<String, Vec<&Fixture>> = std::collections::BTreeMap::new();

    for fixture in fixtures {
        map.entry(fixture.category.clone()).or_default().push(fixture);
    }

    map.into_iter().collect()
}

/// Escape a string for embedding in a Rust raw string or regular string literal.
pub fn escape_rust_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Escape a string for embedding in a Python string literal.
pub fn escape_python_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Escape a string for embedding in a JavaScript/TypeScript string literal.
pub fn escape_js_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
        .replace('`', "\\`")
        .replace('$', "\\$")
}

/// Escape a string for embedding in a Go string literal.
pub fn escape_go_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Escape a string for embedding in a Java string literal.
pub fn escape_java_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Escape a string for embedding in an Elixir string literal.
pub fn escape_elixir_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
        .replace('#', "\\#")
}

/// Escape a string for embedding in a Ruby string literal.
pub fn escape_ruby_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
        .replace('#', "\\#")
}

/// Escape a string for embedding in a C string literal.
pub fn escape_c_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Escape a string for embedding in a PHP single-quoted string literal.
///
/// Only `\` and `'` need escaping in single-quoted PHP strings.
/// Do NOT use for source code with newlines — use `escape_php_source` instead.
pub fn escape_php_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('\'', "\\'")
}

/// Escape a string for embedding in a PHP double-quoted string literal.
///
/// PHP single-quoted strings do NOT interpret `\n`, `\t`, etc.
/// Use double-quoted strings for source code that contains newlines.
pub fn escape_php_source(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('$', "\\$")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Escape a string for embedding in a C# string literal.
pub fn escape_csharp_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Check if a fixture has any language-detection assertions.
pub fn has_detect_assertions(fixture: &Fixture) -> bool {
    fixture.assertions.as_ref().is_some_and(|a| {
        a.detect_from_extension.is_some() || a.detect_from_path.is_some() || a.detect_from_content.is_some()
    })
}

/// Check if a fixture has any ambiguity assertions.
pub fn has_ambiguity_assertions(fixture: &Fixture) -> bool {
    fixture
        .assertions
        .as_ref()
        .is_some_and(|a| a.ambiguity_extension.is_some())
}

/// Check if a fixture has any highlights query assertions.
pub fn has_highlights_assertions(fixture: &Fixture) -> bool {
    fixture
        .assertions
        .as_ref()
        .is_some_and(|a| a.highlights_query_not_empty.is_some() || a.highlights_query_is_none.is_some())
}

/// Check if a fixture has any process-related assertions.
pub fn has_process_assertions(fixture: &Fixture) -> bool {
    fixture.assertions.as_ref().is_some_and(|a| {
        a.process_language.is_some()
            || a.process_structure_count_min.is_some()
            || a.process_structure_contains_kind.is_some()
            || a.process_imports_count_min.is_some()
            || a.process_metrics_total_lines_min.is_some()
            || a.process_metrics_error_count.is_some()
            || a.process_diagnostics_not_empty.is_some()
            || a.process_chunk_count_min.is_some()
            || a.process_comments_count_min.is_some()
            || a.process_exports_count_min.is_some()
            || a.process_imports_contains_source.is_some()
            || a.process_structure_name_contains.is_some()
            || a.process_metrics_code_lines_min.is_some()
            || a.process_metrics_comment_lines_min.is_some()
            || a.process_metrics_max_depth_min.is_some()
    })
}

/// Check if a fixture has chunk-related assertions (requires process with chunking).
pub fn has_chunk_assertions(fixture: &Fixture) -> bool {
    fixture
        .assertions
        .as_ref()
        .is_some_and(|a| a.process_chunk_count_min.is_some())
}
