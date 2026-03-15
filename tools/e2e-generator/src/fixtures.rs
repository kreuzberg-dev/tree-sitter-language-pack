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
    // Intel assertions (process / process with chunking)
    #[serde(default)]
    pub intel_language: Option<String>,
    #[serde(default)]
    pub intel_structure_count_min: Option<usize>,
    #[serde(default)]
    pub intel_structure_contains_kind: Option<String>,
    #[serde(default)]
    pub intel_imports_count_min: Option<usize>,
    #[serde(default)]
    pub intel_metrics_total_lines_min: Option<usize>,
    #[serde(default)]
    pub intel_metrics_error_count: Option<usize>,
    #[serde(default)]
    pub intel_diagnostics_not_empty: Option<bool>,
    #[serde(default)]
    pub intel_chunk_count_min: Option<usize>,
    #[serde(default)]
    pub intel_chunk_max_size: Option<usize>,
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

            let fixture: Fixture =
                serde_json::from_str(&content).map_err(|e| format!("Failed to parse {}: {}", path.display(), e))?;

            fixtures.push(fixture);
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

/// Check if a fixture has any intel-related assertions.
pub fn has_intel_assertions(fixture: &Fixture) -> bool {
    fixture.assertions.as_ref().is_some_and(|a| {
        a.intel_language.is_some()
            || a.intel_structure_count_min.is_some()
            || a.intel_structure_contains_kind.is_some()
            || a.intel_imports_count_min.is_some()
            || a.intel_metrics_total_lines_min.is_some()
            || a.intel_metrics_error_count.is_some()
            || a.intel_diagnostics_not_empty.is_some()
            || a.intel_chunk_count_min.is_some()
    })
}

/// Check if a fixture has chunk-related assertions (requires process with chunking).
pub fn has_chunk_assertions(fixture: &Fixture) -> bool {
    fixture
        .assertions
        .as_ref()
        .is_some_and(|a| a.intel_chunk_count_min.is_some())
}
