/// Language-specific tree-sitter tags queries.
///
/// Captures:
///   @vis    — visibility modifier (e.g. `pub` in Rust)
///   @name   — function/method/class definition name
///   @callee — identifier being called at a call site
///   @exported — JS/TS export statement wrapper (anonymous capture)
///
/// Each language query is compiled once at call time by `ts_pack::run_query`.
/// Queries that fail to compile (wrong node type for a grammar) are silently
/// skipped — safe to add patterns for new languages without breaking existing ones.
use tree_sitter_language_pack as ts_pack;

// ---------------------------------------------------------------------------
// Per-language S-expression query strings
// ---------------------------------------------------------------------------

/// Rust: detect `pub`/`pub(...)` functions and call expressions.
const RUST_TAGS: &str = r#"
(function_item
  (visibility_modifier) @vis
  name: (identifier) @name)

(call_expression
  function: (identifier) @callee)

(call_expression
  function: (field_expression
    field: (identifier) @callee))

(call_expression
  function: (scoped_identifier
    name: (identifier) @callee))
"#;

/// Python: all defs (visibility is by _ convention). Call expressions.
const PYTHON_TAGS: &str = r#"
(function_definition
  name: (identifier) @name)

(call
  function: (identifier) @callee)

(call
  function: (attribute
    attribute: (identifier) @callee))
"#;

/// JavaScript: exported functions (explicit `export` keyword). Call expressions.
const JS_TAGS: &str = r#"
(export_statement
  (function_declaration
    name: (identifier) @name)) @exported

(export_statement
  (lexical_declaration
    (variable_declarator
      name: (identifier) @name
      value: (arrow_function)))) @exported

(export_statement
  (lexical_declaration
    (variable_declarator
      name: (identifier) @name
      value: (function_expression)))) @exported

(call_expression
  function: (identifier) @callee)

(call_expression
  function: (member_expression
    property: (property_identifier) @callee))
"#;

/// Go: all top-level functions (all exported if name starts with uppercase,
/// but we capture all and let the resolver use naming convention).
const GO_TAGS: &str = r#"
(function_declaration
  name: (identifier) @name)

(method_declaration
  name: (field_identifier) @name)

(call_expression
  function: (identifier) @callee)

(call_expression
  function: (selector_expression
    field: (field_identifier) @callee))
"#;

/// Swift: function declarations and call expressions.
const SWIFT_TAGS: &str = r#"
(function_declaration
  name: (simple_identifier) @name)

(function_declaration
  name: (identifier) @name)

(call_expression
  (simple_identifier) @callee
  (call_suffix))

(call_expression
  (identifier) @callee
  (call_suffix))

(call_expression
  (navigation_expression
    (navigation_suffix
      (simple_identifier) @callee))
  (call_suffix))
"#;

/// TypeScript/TSX: same as JS plus type-annotated export forms.
const TS_TAGS: &str = r#"
(export_statement
  (function_declaration
    name: (identifier) @name)) @exported

(export_statement
  (lexical_declaration
    (variable_declarator
      name: (identifier) @name
      value: [(arrow_function)(function_expression)]))) @exported

(call_expression
  function: (identifier) @callee)

(call_expression
  function: (member_expression
    property: (property_identifier) @callee))
"#;

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// A single resolved call expression with its byte offset in the source.
pub struct CallSite {
    /// Byte offset of the call expression start (for enclosing-scope lookup).
    pub start_byte: usize,
    /// Name of the function/method being called.
    pub callee: String,
}

/// Result of running the tags query on a single file.
pub struct TagsResult {
    /// Names of functions/classes that are exported (public API surface).
    pub exported_names: std::collections::HashSet<String>,
    /// All call sites found in this file (with byte position for scope lookup).
    pub call_sites: Vec<CallSite>,
}

/// Run the tags query for `lang_name` against the already-parsed `tree`.
///
/// Returns `None` if there is no query configured for this language, or if
/// query compilation fails (e.g. the grammar uses different node type names).
pub fn run_tags(lang_name: &str, tree: &ts_pack::Tree, source: &[u8]) -> Option<TagsResult> {
    let query_str = tags_query(lang_name)?;

    // Split the multi-pattern query into individual patterns and try each one.
    // This way a single bad pattern doesn't kill the whole query for a language.
    let patterns = split_query_patterns(query_str);
    if patterns.is_empty() {
        return None;
    }

    let mut exported_names = std::collections::HashSet::new();
    let mut call_sites = Vec::new();

    for pattern in &patterns {
        let matches = match ts_pack::run_query(tree, lang_name, pattern, source) {
            Ok(m) => m,
            Err(_) => continue, // invalid node type for this grammar — skip
        };

        for m in &matches {
            let is_export_pattern = m.captures.iter().any(|(cap, _)| cap == "exported");
            let mut has_vis = false;
            let mut def_name: Option<String> = None;
            // (start_byte, callee_name)
            let mut callee_site: Option<(usize, String)> = None;

            for (cap_name, node_info) in &m.captures {
                let text = match ts_pack::extract_text(source, node_info) {
                    Ok(t) => t.to_string(),
                    Err(_) => continue,
                };

                match cap_name.as_str() {
                    "vis" => {
                        has_vis = true;
                    }
                    "name" => {
                        def_name = Some(text);
                    }
                    "callee" => {
                        callee_site = Some((node_info.start_byte, text));
                    }
                    _ => {}
                }
            }

            if let Some(name) = def_name {
                if has_vis || is_export_pattern {
                    exported_names.insert(name);
                }
            }

            if let Some((start_byte, callee)) = callee_site {
                call_sites.push(CallSite { start_byte, callee });
            }
        }
    }

    Some(TagsResult {
        exported_names,
        call_sites,
    })
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn tags_query(lang: &str) -> Option<&'static str> {
    match lang {
        "rust" => Some(RUST_TAGS),
        "python" => Some(PYTHON_TAGS),
        "javascript" => Some(JS_TAGS),
        "typescript" | "tsx" => Some(TS_TAGS),
        "go" => Some(GO_TAGS),
        "swift" => Some(SWIFT_TAGS),
        _ => None,
    }
}

/// Naively split a multi-pattern query string into individual S-expression
/// patterns. Splits on blank lines between top-level patterns so that each
/// `run_query` call compiles exactly one pattern — allowing graceful failure.
fn split_query_patterns(query: &str) -> Vec<String> {
    let mut patterns = Vec::new();
    let mut current = String::new();
    let mut depth = 0i32;

    for line in query.lines() {
        let trimmed = line.trim();
        for ch in trimmed.chars() {
            if ch == '(' {
                depth += 1;
            } else if ch == ')' {
                depth -= 1;
            }
        }
        current.push_str(line);
        current.push('\n');

        // A complete top-level pattern: depth returns to 0 after a non-empty chunk.
        if depth == 0 && !current.trim().is_empty() {
            let pat = current.trim().to_string();
            if !pat.is_empty() && !pat.starts_with(';') {
                patterns.push(pat);
            }
            current.clear();
        }
    }

    // Push any remaining content
    let remainder = current.trim().to_string();
    if !remainder.is_empty() {
        patterns.push(remainder);
    }

    patterns
}
