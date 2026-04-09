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
use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::sync::{Arc, LazyLock, Mutex, RwLock};
use std::time::Instant;

use tree_sitter_language_pack as ts_pack;

static VALID_TAGS_QUERY_CACHE: LazyLock<RwLock<HashMap<String, Arc<String>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));
static VALID_TAGS_QUERY_PROFILE: LazyLock<Mutex<HashMap<(String, String), ValidTagsQueryAggregate>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static QUERY_PROFILE_BY_LABEL: LazyLock<Mutex<HashMap<(String, String), QueryProfileAggregate>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static QUERY_PROFILE_BY_FILE: LazyLock<Mutex<HashMap<(String, String, String), QueryProfileAggregate>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Debug, Clone, Copy, Default)]
struct QueryProfileAggregate {
    runs: usize,
    prepared_runs: usize,
    query_text_runs: usize,
    total_matches: usize,
    total_lookup_secs: f64,
    total_elapsed_secs: f64,
    total_process_secs: f64,
    total_wall_secs: f64,
    max_matches: usize,
    max_lookup_secs: f64,
    max_elapsed_secs: f64,
    max_process_secs: f64,
    max_wall_secs: f64,
    exceeded_match_limit_count: usize,
}

impl QueryProfileAggregate {
    fn record(
        &mut self,
        profile: ts_pack::QueryProfile,
        process_secs: f64,
        wall_secs: f64,
        source_kind: QuerySourceKind,
    ) {
        self.runs += 1;
        match source_kind {
            QuerySourceKind::Prepared => self.prepared_runs += 1,
            QuerySourceKind::QueryText => self.query_text_runs += 1,
        }
        self.total_matches += profile.match_count;
        self.total_lookup_secs += profile.lookup_secs;
        self.total_elapsed_secs += profile.elapsed_secs;
        self.total_process_secs += process_secs;
        self.total_wall_secs += wall_secs;
        self.max_matches = self.max_matches.max(profile.match_count);
        self.max_lookup_secs = self.max_lookup_secs.max(profile.lookup_secs);
        self.max_elapsed_secs = self.max_elapsed_secs.max(profile.elapsed_secs);
        self.max_process_secs = self.max_process_secs.max(process_secs);
        self.max_wall_secs = self.max_wall_secs.max(wall_secs);
        if profile.exceeded_match_limit {
            self.exceeded_match_limit_count += 1;
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct QueryProfileSummaryRow {
    pub(crate) lang: String,
    pub(crate) label: String,
    pub(crate) file_path: Option<String>,
    pub(crate) runs: usize,
    pub(crate) prepared_runs: usize,
    pub(crate) query_text_runs: usize,
    pub(crate) total_matches: usize,
    pub(crate) total_lookup_secs: f64,
    pub(crate) total_elapsed_secs: f64,
    pub(crate) total_process_secs: f64,
    pub(crate) total_wall_secs: f64,
    pub(crate) max_matches: usize,
    pub(crate) max_lookup_secs: f64,
    pub(crate) max_elapsed_secs: f64,
    pub(crate) max_process_secs: f64,
    pub(crate) max_wall_secs: f64,
    pub(crate) exceeded_match_limit_count: usize,
}

pub(crate) fn reset_query_profile_aggregates() {
    if let Ok(mut agg) = VALID_TAGS_QUERY_PROFILE.lock() {
        agg.clear();
    }
    if let Ok(mut agg) = QUERY_PROFILE_BY_LABEL.lock() {
        agg.clear();
    }
    if let Ok(mut agg) = QUERY_PROFILE_BY_FILE.lock() {
        agg.clear();
    }
}

pub(crate) fn summarize_valid_tags_query_aggregates() -> Vec<ValidTagsQuerySummaryRow> {
    VALID_TAGS_QUERY_PROFILE
        .lock()
        .ok()
        .map(|agg| {
            agg.iter()
                .map(|((lang, cache_key), stats)| ValidTagsQuerySummaryRow {
                    lang: lang.clone(),
                    cache_key: cache_key.clone(),
                    hits: stats.hits,
                    misses: stats.misses,
                    total_secs: stats.total_secs,
                    max_secs: stats.max_secs,
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

pub(crate) fn summarize_query_profile_aggregates() -> (Vec<QueryProfileSummaryRow>, Vec<QueryProfileSummaryRow>) {
    let by_label = QUERY_PROFILE_BY_LABEL
        .lock()
        .ok()
        .map(|agg| {
            agg.iter()
                .map(|((lang, label), stats)| QueryProfileSummaryRow {
                    lang: lang.clone(),
                    label: label.clone(),
                    file_path: None,
                    runs: stats.runs,
                    prepared_runs: stats.prepared_runs,
                    query_text_runs: stats.query_text_runs,
                    total_matches: stats.total_matches,
                    total_lookup_secs: stats.total_lookup_secs,
                    total_elapsed_secs: stats.total_elapsed_secs,
                    total_process_secs: stats.total_process_secs,
                    total_wall_secs: stats.total_wall_secs,
                    max_matches: stats.max_matches,
                    max_lookup_secs: stats.max_lookup_secs,
                    max_elapsed_secs: stats.max_elapsed_secs,
                    max_process_secs: stats.max_process_secs,
                    max_wall_secs: stats.max_wall_secs,
                    exceeded_match_limit_count: stats.exceeded_match_limit_count,
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let by_file = QUERY_PROFILE_BY_FILE
        .lock()
        .ok()
        .map(|agg| {
            agg.iter()
                .map(|((lang, label, file_path), stats)| QueryProfileSummaryRow {
                    lang: lang.clone(),
                    label: label.clone(),
                    file_path: Some(file_path.clone()),
                    runs: stats.runs,
                    prepared_runs: stats.prepared_runs,
                    query_text_runs: stats.query_text_runs,
                    total_matches: stats.total_matches,
                    total_lookup_secs: stats.total_lookup_secs,
                    total_elapsed_secs: stats.total_elapsed_secs,
                    total_process_secs: stats.total_process_secs,
                    total_wall_secs: stats.total_wall_secs,
                    max_matches: stats.max_matches,
                    max_lookup_secs: stats.max_lookup_secs,
                    max_elapsed_secs: stats.max_elapsed_secs,
                    max_process_secs: stats.max_process_secs,
                    max_wall_secs: stats.max_wall_secs,
                    exceeded_match_limit_count: stats.exceeded_match_limit_count,
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    (by_label, by_file)
}

fn record_query_profile(
    lang_name: &str,
    query_label: &str,
    file_path: &str,
    profile: ts_pack::QueryProfile,
    process_secs: f64,
    wall_secs: f64,
    source_kind: QuerySourceKind,
) {
    if let Ok(mut agg) = QUERY_PROFILE_BY_LABEL.lock() {
        agg.entry((lang_name.to_string(), query_label.to_string()))
            .or_default()
            .record(profile, process_secs, wall_secs, source_kind);
    }
    if let Ok(mut agg) = QUERY_PROFILE_BY_FILE.lock() {
        agg.entry((lang_name.to_string(), query_label.to_string(), file_path.to_string()))
            .or_default()
            .record(profile, process_secs, wall_secs, source_kind);
    }
}

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
    object: (identifier) @recv
    attribute: (identifier) @callee))

(call
  function: (attribute
    object: (identifier) @launch_module
    attribute: (identifier) @launch_callee)
  arguments: (argument_list
    (list
      (string) @launch_arg)))
"#;

const PYTHON_LAUNCH_ASSIGN_TAGS: &str = r#"
(assignment
  left: (identifier) @launch_assign_name
  right: (list (string) @launch_assign_str)) @launch_assign_stmt

(assignment
  left: (identifier) @launch_assign_name
  right: (tuple (string) @launch_assign_str)) @launch_assign_stmt

(assignment
  left: (identifier) @launch_assign_name
  right: (list
    (call
      function: (attribute) @launch_join_fn
      arguments: (argument_list (string) @launch_join_arg)) @launch_join_call)) @launch_assign_stmt

(assignment
  left: (identifier) @launch_assign_name
  right: (tuple
    (call
      function: (attribute) @launch_join_fn
      arguments: (argument_list (string) @launch_join_arg)) @launch_join_call)) @launch_assign_stmt
"#;

const PYTHON_LAUNCH_IDENT_CALL_TAGS: &str = r#"
(call
  function: (attribute
    object: (identifier) @launch_module
    attribute: (identifier) @launch_callee)
  arguments: (argument_list (identifier) @launch_arg_ident)) @launch_call

(call
  function: (attribute
    object: (identifier) @launch_module
    attribute: (identifier) @launch_callee)
  arguments: (argument_list
    (keyword_argument value: (identifier) @launch_arg_ident))) @launch_call
"#;

const JS_CALL_TAGS: &str = r#"
(call_expression
  function: (identifier) @callee)

(call_expression
  function: (member_expression
    property: (property_identifier) @callee))
"#;

const JS_TS_DB_TAGS: &str = r#"
(member_expression
  object: (member_expression
    object: (identifier) @dbobj
    property: [(property_identifier) (identifier)] @db)
  property: [(property_identifier) (identifier)] @db_method)
(#any-of? @dbobj "prisma" "tx" "prismaClient" "db")

(member_expression
  object: (member_expression
    object: (member_expression
      object: (this)
      property: [(property_identifier) (identifier)] @dbobj)
    property: [(property_identifier) (identifier)] @db)
  property: [(property_identifier) (identifier)] @db_method)
(#eq? @dbobj "prisma")

(member_expression
  object: (member_expression
    object: (member_expression
      property: [(property_identifier) (identifier)] @dbobj)
    property: [(property_identifier) (identifier)] @db)
  property: [(property_identifier) (identifier)] @db_method)
(#eq? @dbobj "prisma")

(member_expression
  object: (member_expression
    object: (member_expression
      object: (identifier) @ctx
      property: [(property_identifier) (identifier)] @dbobj)
    property: [(property_identifier) (identifier)] @db)
  property: [(property_identifier) (identifier)] @db_method)
(#eq? @dbobj "prisma")

(member_expression
  object: (member_expression
    object: (member_expression
      object: (identifier) @ctx
      property: [(property_identifier) (identifier)] @dbobj)
    property: [(property_identifier) (identifier)] @db)
  property: [(property_identifier) (identifier)] @db_method)
(#match? @dbobj ".*Prisma$")
"#;

const JS_TS_EXTERNAL_TAGS: &str = r#"
(call_expression
  function: (identifier) @external_callee
  arguments: (arguments (string) @external_arg))
(#any-of? @external_callee "fetch" "axios" "ky" "ofetch" "$fetch")

(call_expression
  function: (identifier) @external_callee
  arguments: (arguments (identifier) @external_arg))
(#any-of? @external_callee "fetch" "axios" "ky" "ofetch" "$fetch")

(call_expression
  function: (identifier) @external_callee
  arguments: (arguments (template_string) @external_arg))
(#any-of? @external_callee "fetch" "axios" "ky" "ofetch" "$fetch")

(call_expression
  function: (member_expression
    object: (identifier) @external_callee)
  arguments: (arguments (string) @external_arg))
(#any-of? @external_callee "fetch" "axios" "ky" "ofetch" "$fetch")

(call_expression
  function: (member_expression
    object: (identifier) @external_callee)
  arguments: (arguments (identifier) @external_arg))
(#any-of? @external_callee "fetch" "axios" "ky" "ofetch" "$fetch")

(call_expression
  function: (member_expression
    object: (identifier) @external_callee)
  arguments: (arguments (template_string) @external_arg))
(#any-of? @external_callee "fetch" "axios" "ky" "ofetch" "$fetch")

(call_expression
  function: (identifier) @external_callee
  arguments: (arguments
    (binary_expression
      left: (identifier) @external_arg_left
      operator: "+"
      right: (string) @external_arg_right)))
(#any-of? @external_callee "fetch" "axios" "ky" "ofetch" "$fetch")

(call_expression
  function: (identifier) @external_callee
  arguments: (arguments
    (binary_expression
      left: (string) @external_arg_left
      operator: "+"
      right: (identifier) @external_arg_right)))
(#any-of? @external_callee "fetch" "axios" "ky" "ofetch" "$fetch")

(call_expression
  function: (member_expression
    object: (identifier) @external_callee)
  arguments: (arguments
    (binary_expression
      left: (identifier) @external_arg_left
      operator: "+"
      right: (string) @external_arg_right)))
(#any-of? @external_callee "fetch" "axios" "ky" "ofetch" "$fetch")

(call_expression
  function: (member_expression
    object: (identifier) @external_callee)
  arguments: (arguments
    (binary_expression
      left: (string) @external_arg_left
      operator: "+"
      right: (identifier) @external_arg_right)))
(#any-of? @external_callee "fetch" "axios" "ky" "ofetch" "$fetch")

(call_expression
  function: (identifier) @external_callee
  arguments: (arguments
    (new_expression
      constructor: (identifier) @external_url_ctor
      arguments: (arguments (string) @external_url_path (string) @external_url_base))))
(#any-of? @external_callee "fetch" "axios" "ky" "ofetch" "$fetch")

(call_expression
  function: (identifier) @external_callee
  arguments: (arguments
    (new_expression
      constructor: (identifier) @external_url_ctor
      arguments: (arguments (string) @external_url_path (identifier) @external_url_base_ident))))
(#any-of? @external_callee "fetch" "axios" "ky" "ofetch" "$fetch")

(call_expression
  function: (member_expression
    object: (identifier) @external_callee)
  arguments: (arguments
    (new_expression
      constructor: (identifier) @external_url_ctor
      arguments: (arguments (string) @external_url_path (string) @external_url_base))))
(#any-of? @external_callee "fetch" "axios" "ky" "ofetch" "$fetch")

(call_expression
  function: (member_expression
    object: (identifier) @external_callee)
  arguments: (arguments
    (new_expression
      constructor: (identifier) @external_url_ctor
      arguments: (arguments (string) @external_url_path (identifier) @external_url_base_ident))))
(#any-of? @external_callee "fetch" "axios" "ky" "ofetch" "$fetch")
"#;

const JS_TS_CONST_TAGS: &str = r#"
(lexical_declaration
  (variable_declarator
    name: (identifier) @const_name
    value: (string) @const_value))

(lexical_declaration
  (variable_declarator
    name: (identifier) @const_name
    value: (template_string) @const_value))

(lexical_declaration
  (variable_declarator
    name: (identifier) @const_name
    value: (binary_expression
      left: (string) @const_left
      operator: "+"
      right: (string) @const_right)))

(lexical_declaration
  (variable_declarator
    name: (identifier) @const_name
    value: (member_expression
      object: (member_expression
        object: (identifier) @env_root
        property: (property_identifier) @env_prop)
      property: (property_identifier) @env_key)))

(lexical_declaration
  (variable_declarator
    name: (identifier) @const_name
    value: (member_expression
      object: (member_expression
        object: (member_expression
          object: (identifier) @env_import
          property: (property_identifier) @env_meta)
        property: (property_identifier) @env_env)
      property: (property_identifier) @env_key)))
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

/// Swift: exported declarations and call expressions.
const SWIFT_TAGS: &str = r#"
(function_declaration
  (modifiers) @vis
  name: (simple_identifier) @name) @exported

(function_declaration
  (modifiers) @vis
  name: (identifier) @name) @exported

(class_declaration
  (modifiers) @vis
  name: (type_identifier) @name) @exported

(struct_declaration
  (modifiers) @vis
  name: (type_identifier) @name) @exported

(enum_declaration
  (modifiers) @vis
  name: (type_identifier) @name) @exported

(protocol_declaration
  (modifiers) @vis
  name: (type_identifier) @name) @exported

(typealias_declaration
  (modifiers) @vis
  name: (type_identifier) @name) @exported

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
    target: (self_expression) @recv
    (navigation_suffix
      (simple_identifier) @callee))
  (call_suffix))

(call_expression
  (navigation_expression
    target: (navigation_expression
      target: (self_expression) @recv)
    (navigation_suffix
      (simple_identifier) @callee))
  (call_suffix))

(call_expression
  (navigation_expression
    target: (simple_identifier) @recv
    (navigation_suffix
      (simple_identifier) @callee))
  (call_suffix))

(call_expression
  (navigation_expression
    target: (identifier) @recv
    (navigation_suffix
      (simple_identifier) @callee))
  (call_suffix))
"#;

const TS_CALL_TAGS: &str = r#"
(call_expression
  function: (identifier) @callee)

(call_expression
  function: (member_expression
    property: (property_identifier) @callee))
"#;

fn strip_string_literal(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.len() < 2 {
        return None;
    }
    let first = trimmed.chars().next()?;
    let last = trimmed.chars().last()?;
    let is_quote = (first == '"' && last == '"') || (first == '\'' && last == '\'') || (first == '`' && last == '`');
    if !is_quote {
        return None;
    }
    let inner = &trimmed[1..trimmed.len() - 1];
    if inner.contains("${") {
        return None;
    }
    Some(inner.to_string())
}

fn parse_simple_template_arg(raw: &str) -> Option<ExternalCallArg> {
    let trimmed = raw.trim();
    if !(trimmed.starts_with('`') && trimmed.ends_with('`')) {
        return None;
    }
    let inner = &trimmed[1..trimmed.len() - 1];
    let start = inner.find("${")?;
    let ident_start = start + 2;
    let ident_end_rel = inner[ident_start..].find('}')?;
    let ident_end = ident_start + ident_end_rel;
    if inner[ident_end + 1..].contains("${") {
        return None;
    }

    let ident = inner[ident_start..ident_end].trim();
    if ident.is_empty()
        || !ident
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '$'))
    {
        return None;
    }

    let prefix = &inner[..start];
    let suffix = &inner[ident_end + 1..];
    match (!prefix.is_empty(), !suffix.is_empty()) {
        (false, false) => Some(ExternalCallArg::Identifier(ident.to_string())),
        (false, true) => Some(ExternalCallArg::ConcatIdentLiteral {
            ident: ident.to_string(),
            literal: suffix.to_string(),
        }),
        (true, false) => Some(ExternalCallArg::ConcatLiteralIdent {
            literal: prefix.to_string(),
            ident: ident.to_string(),
        }),
        (true, true) => None,
    }
}

fn is_delegate_property_use(source: &[u8], end_byte: usize) -> bool {
    let mut idx = end_byte;
    while idx < source.len() {
        match source[idx] {
            b' ' | b'\t' | b'\r' | b'\n' => idx += 1,
            b'.' => return true,
            b'?' if idx + 1 < source.len() && source[idx + 1] == b'.' => return true,
            _ => return false,
        }
    }
    false
}

fn is_launch_callee(module: &str, callee: &str) -> bool {
    module == "subprocess" && matches!(callee, "run" | "Popen" | "call")
}

fn join_path_parts(parts: &[String]) -> Option<String> {
    if parts.is_empty() {
        return None;
    }
    let mut out = String::new();
    for part in parts {
        let trimmed = part.trim_matches('/');
        if trimmed.is_empty() {
            continue;
        }
        if !out.is_empty() {
            out.push('/');
        }
        out.push_str(trimmed);
    }
    if out.is_empty() { None } else { Some(out) }
}

fn is_join_fn(text: &str) -> bool {
    text.ends_with(".path.join") || text == "path.join" || text == "os.path.join"
}

fn debug_launch_enabled() -> bool {
    std::env::var("TS_PACK_DEBUG_LAUNCH")
        .ok()
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

#[derive(Clone, Copy)]
struct ScopeRange {
    start: usize,
    end: usize,
}

fn scope_for(node_start: usize, node_end: usize, scopes: &[ScopeRange]) -> usize {
    let mut best_idx = 0usize;
    let mut best_span = usize::MAX;
    for (idx, scope) in scopes.iter().enumerate() {
        if node_start >= scope.start && node_end <= scope.end {
            let span = scope.end.saturating_sub(scope.start);
            if span < best_span {
                best_idx = idx;
                best_span = span;
            }
        }
    }
    best_idx
}

fn resolve_python_launch_idents(tree: &ts_pack::Tree, source: &[u8]) -> Vec<String> {
    let mut scopes: Vec<ScopeRange> = Vec::new();
    if let Some(query) = ts_pack::get_locals_query("python") {
        match ts_pack::run_query(tree, "python", query, source) {
            Ok(matches) => {
                for m in matches {
                    for (cap, info) in m.captures {
                        if cap.as_ref() == "local.scope" {
                            scopes.push(ScopeRange {
                                start: info.start_byte,
                                end: info.end_byte,
                            });
                        }
                    }
                }
            }
            Err(err) => {
                if debug_launch_enabled() {
                    eprintln!("[ts-pack-index] launch locals query failed: {err}");
                }
            }
        }
    }
    if scopes.is_empty() {
        scopes.push(ScopeRange {
            start: 0,
            end: source.len(),
        });
    }

    let mut assignments: HashMap<(String, usize), Vec<(usize, Vec<String>)>> = HashMap::new();
    let mut join_calls: HashMap<(String, usize, usize), (Option<String>, Vec<String>, usize)> = HashMap::new();
    match ts_pack::run_query(tree, "python", PYTHON_LAUNCH_ASSIGN_TAGS, source) {
        Ok(matches) => {
            for m in matches {
                let mut name: Option<String> = None;
                let mut stmt: Option<ts_pack::NodeInfo> = None;
                let mut assign_strings: Vec<String> = Vec::new();
                let mut join_fn: Option<String> = None;
                let mut join_arg: Option<String> = None;
                let mut join_call: Option<ts_pack::NodeInfo> = None;

                for (cap, info) in m.captures {
                    match cap.as_ref() {
                        "launch_assign_name" => {
                            if let Ok(text) = ts_pack::extract_text(source, &info) {
                                name = Some(text.to_string());
                            }
                        }
                        "launch_assign_stmt" => {
                            stmt = Some(info);
                        }
                        "launch_assign_str" => {
                            if let Ok(text) = ts_pack::extract_text(source, &info) {
                                if let Some(literal) = strip_string_literal(text) {
                                    assign_strings.push(literal);
                                }
                            }
                        }
                        "launch_join_fn" => {
                            if let Ok(text) = ts_pack::extract_text(source, &info) {
                                join_fn = Some(text.to_string());
                            }
                        }
                        "launch_join_arg" => {
                            if let Ok(text) = ts_pack::extract_text(source, &info) {
                                if let Some(literal) = strip_string_literal(text) {
                                    join_arg = Some(literal);
                                }
                            }
                        }
                        "launch_join_call" => {
                            join_call = Some(info);
                        }
                        _ => {}
                    }
                }

                let Some(name) = name else {
                    continue;
                };
                let Some(stmt) = stmt else {
                    continue;
                };
                let stmt_start = stmt.start_byte;
                let scope_id = scope_for(stmt.start_byte, stmt.end_byte, &scopes);

                if let Some(literal) = join_arg {
                    let call = join_call.unwrap_or(stmt.clone());
                    let key = (name.clone(), scope_id, call.start_byte);
                    let entry = join_calls.entry(key).or_insert((None, Vec::new(), stmt_start));
                    if entry.0.is_none() {
                        entry.0 = join_fn.clone();
                    }
                    entry.1.push(literal);
                }

                if !assign_strings.is_empty() {
                    let mut paths: Vec<String> = Vec::new();
                    for literal in assign_strings {
                        if literal.ends_with(".py") {
                            paths.push(literal);
                        }
                    }
                    if !paths.is_empty() {
                        assignments
                            .entry((name, scope_id))
                            .or_default()
                            .push((stmt.start_byte, paths));
                    }
                }
            }
        }
        Err(err) => {
            if debug_launch_enabled() {
                eprintln!("[ts-pack-index] launch assign query failed: {err}");
            }
        }
    }

    for ((name, scope_id, _), (fn_text, args, stmt_start)) in join_calls {
        let Some(fn_text) = fn_text else {
            continue;
        };
        if !is_join_fn(&fn_text) {
            continue;
        }
        let Some(path) = join_path_parts(&args) else {
            continue;
        };
        if !path.ends_with(".py") {
            continue;
        }
        assignments
            .entry((name, scope_id))
            .or_default()
            .push((stmt_start, vec![path]));
    }

    for list in assignments.values_mut() {
        list.sort_by_key(|(start, _)| *start);
    }

    let mut calls: Vec<(String, usize, usize)> = Vec::new();
    match ts_pack::run_query(tree, "python", PYTHON_LAUNCH_IDENT_CALL_TAGS, source) {
        Ok(matches) => {
            for m in matches {
                let mut module: Option<String> = None;
                let mut callee: Option<String> = None;
                let mut arg_ident: Option<String> = None;
                let mut call_node: Option<ts_pack::NodeInfo> = None;

                for (cap, info) in m.captures {
                    match cap.as_ref() {
                        "launch_module" => {
                            if let Ok(text) = ts_pack::extract_text(source, &info) {
                                module = Some(text.to_string());
                            }
                        }
                        "launch_callee" => {
                            if let Ok(text) = ts_pack::extract_text(source, &info) {
                                callee = Some(text.to_string());
                            }
                        }
                        "launch_arg_ident" => {
                            if let Ok(text) = ts_pack::extract_text(source, &info) {
                                arg_ident = Some(text.to_string());
                            }
                        }
                        "launch_call" => {
                            call_node = Some(info);
                        }
                        _ => {}
                    }
                }

                let (Some(module), Some(callee), Some(arg_ident), Some(call_node)) =
                    (module, callee, arg_ident, call_node)
                else {
                    continue;
                };
                if !is_launch_callee(&module, &callee) {
                    continue;
                }
                let scope_id = scope_for(call_node.start_byte, call_node.end_byte, &scopes);
                calls.push((arg_ident, scope_id, call_node.start_byte));
            }
        }
        Err(err) => {
            if debug_launch_enabled() {
                eprintln!("[ts-pack-index] launch call query failed: {err}");
            }
        }
    }

    let mut out: Vec<String> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    let call_count = calls.len();
    for (ident, scope_id, call_start) in calls {
        let mut resolved: Vec<String> = Vec::new();
        if let Some(list) = assignments.get(&(ident.clone(), scope_id)) {
            for (start, paths) in list.iter().rev() {
                if *start <= call_start {
                    resolved = paths.clone();
                    break;
                }
            }
        }
        if resolved.is_empty() && scope_id != 0 {
            if let Some(list) = assignments.get(&(ident.clone(), 0)) {
                for (start, paths) in list.iter().rev() {
                    if *start <= call_start {
                        resolved = paths.clone();
                        break;
                    }
                }
            }
        }

        for path in resolved {
            if seen.insert(path.clone()) {
                out.push(path);
            }
        }
    }

    if debug_launch_enabled() {
        eprintln!(
            "[ts-pack-index] launch resolve: scopes={} assigns={} calls={} resolved={}",
            scopes.len(),
            assignments.len(),
            call_count,
            out.len()
        );
    }

    out
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// A single resolved call expression with its byte offset in the source.
#[derive(Clone)]
pub struct CallSite {
    /// Byte offset of the call expression start (for enclosing-scope lookup).
    pub start_byte: usize,
    /// Name of the function/method being called.
    pub callee: String,
    /// Receiver identifier for member calls.
    pub receiver: Option<String>,
}

#[derive(Clone)]
pub enum ExternalCallArg {
    Literal(String),
    Identifier(String),
    ConcatIdentLiteral { ident: String, literal: String },
    ConcatLiteralIdent { literal: String, ident: String },
    UrlLiteral { path: String, base: String },
    UrlWithBaseIdent { path: String, base_ident: String },
}

#[derive(Clone)]
pub struct ExternalCallSite {
    pub arg: ExternalCallArg,
}

/// Result of running the tags query on a single file.
pub struct TagsResult {
    /// Names of functions/classes that are exported (public API surface).
    pub exported_names: std::collections::HashSet<String>,
    /// All call sites found in this file (with byte position for scope lookup).
    pub call_sites: Vec<CallSite>,
    /// DB model references (currently Prisma delegates for ts/js).
    pub db_models: std::collections::HashSet<String>,
    /// External API call sites (js/ts only).
    pub external_calls: Vec<ExternalCallSite>,
    /// Constant string assignments (js/ts only).
    pub const_strings: std::collections::HashMap<String, String>,
    /// Subprocess launch script paths (python only).
    pub launch_calls: Vec<String>,
}

#[derive(Clone, Default)]
pub struct TagQueryBundle {
    queries: Vec<(String, TagQuerySource)>,
}

impl TagQueryBundle {
    fn from_queries(queries: Vec<(String, TagQuerySource)>) -> Self {
        Self { queries }
    }

    fn as_slice(&self) -> &[(String, TagQuerySource)] {
        &self.queries
    }
}

#[derive(Clone)]
enum TagQuerySource {
    QueryText(Arc<String>),
    Prepared(ts_pack::PreparedQuery),
}

#[derive(Clone, Copy)]
enum QuerySourceKind {
    QueryText,
    Prepared,
}

#[derive(Clone, Default)]
pub struct BatchTagQueryBundles {
    typescript: Option<TagQueryBundle>,
    javascript: Option<TagQueryBundle>,
}

impl BatchTagQueryBundles {
    pub fn for_lang_and_source(&self, lang: &str, source: &[u8]) -> Option<TagQueryBundle> {
        match lang {
            "javascript" => self.javascript.as_ref().map(|bundle| filter_js_ts_bundle(bundle, source)),
            "typescript" | "tsx" => self.typescript.as_ref().map(|bundle| filter_js_ts_bundle(bundle, source)),
            _ => None,
        }
    }
}

/// Run the tags query for `lang_name` against the already-parsed `tree`.
///
/// Returns `None` if there is no query configured for this language, or if
/// query compilation fails (e.g. the grammar uses different node type names).
pub fn run_tags(
    lang_name: &str,
    tree: &ts_pack::Tree,
    source: &[u8],
    file_path: &str,
    batch_bundle: Option<&TagQueryBundle>,
) -> Option<TagsResult> {
    let mut exported_names = std::collections::HashSet::new();
    let mut call_sites = Vec::new();
    let mut db_models = std::collections::HashSet::new();
    let mut external_calls = Vec::new();
    let mut const_strings = std::collections::HashMap::new();
    let mut launch_calls = Vec::new();

    let is_external_callee = |name: &str| matches!(name, "fetch" | "axios" | "ky" | "ofetch" | "$fetch");

    let debug_tag_source = std::env::var("TS_PACK_DEBUG_TAG_SOURCE")
        .ok()
        .map(|v| matches!(v.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false);
    let query_sources = match batch_bundle {
        Some(bundle) => bundle.as_slice().to_vec(),
        None => query_sources_for(lang_name, source)?,
    };
    if debug_tag_source
        && (file_path.ends_with("QuickBooksService.ts")
            || file_path.ends_with("financials.js")
            || file_path.ends_with("landlord-dashboard.js"))
    {
        let source_mode = if batch_bundle.is_some() { "batch" } else { "fallback" };
        let kinds = query_sources
            .iter()
            .map(|(label, src)| {
                format!(
                    "{}:{}",
                    label,
                    match src {
                        TagQuerySource::Prepared(_) => "prepared",
                        TagQuerySource::QueryText(_) => "text",
                    }
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        eprintln!(
            "[ts-pack-index:tag-source] lang={lang_name} file={file_path} mode={source_mode} queries=[{kinds}]"
        );
    }
    let mut saw_match = false;
    let profile_queries = std::env::var("TS_PACK_DEBUG_QUERY_PROFILE")
        .ok()
        .map(|v| matches!(v.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false);
    let profile_query_lines = std::env::var("TS_PACK_DEBUG_QUERY_PROFILE_LINES")
        .ok()
        .map(|v| matches!(v.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false);
    for (query_label, query_src) in &query_sources {
        let wall_started = if profile_queries { Some(Instant::now()) } else { None };
        let source_kind = match query_src {
            TagQuerySource::QueryText(_) => QuerySourceKind::QueryText,
            TagQuerySource::Prepared(_) => QuerySourceKind::Prepared,
        };
        let byte_ranges = if query_label == "js-ts:external" {
            external_query_ranges(source)
        } else {
            None
        };
        let (matches, profile) = if profile_queries {
            match query_src {
                TagQuerySource::QueryText(query_str) => match run_query_profiled_with_optional_ranges(
                    tree,
                    lang_name,
                    query_str.as_str(),
                    source,
                    byte_ranges.as_deref(),
                ) {
                    Ok(result) => result,
                    Err(_) => continue,
                },
                TagQuerySource::Prepared(prepared) => match run_prepared_query_profiled_with_optional_ranges(
                    tree,
                    prepared,
                    source,
                    byte_ranges.as_deref(),
                ) {
                    Ok((matches, mut profile)) => {
                        profile.lookup_secs = 0.0;
                        (matches, profile)
                    }
                    Err(_) => continue,
                },
            }
        } else {
            match query_src {
                TagQuerySource::QueryText(query_str) => match run_query_with_optional_ranges(
                    tree,
                    lang_name,
                    query_str.as_str(),
                    source,
                    byte_ranges.as_deref(),
                ) {
                    Ok(matches) => (matches, ts_pack::QueryProfile::default()),
                    Err(_) => continue,
                },
                TagQuerySource::Prepared(prepared) => match run_prepared_query_with_optional_ranges(
                    tree,
                    prepared,
                    source,
                    byte_ranges.as_deref(),
                ) {
                    Ok(matches) => (matches, ts_pack::QueryProfile::default()),
                    Err(_) => continue,
                },
            }
        };
        if !matches.is_empty() {
            saw_match = true;
        }
        if profile_queries && profile_query_lines && (profile.exceeded_match_limit || profile.elapsed_secs >= 0.010) {
            eprintln!(
                "[ts-pack-index:query] lang={lang_name} label={} file={} matches={} exceeded_match_limit={} elapsed_ms={:.2}",
                query_label,
                file_path,
                profile.match_count,
                profile.exceeded_match_limit,
                profile.elapsed_secs * 1000.0,
            );
        }
        let process_started = if profile_queries { Some(Instant::now()) } else { None };
        for m in &matches {
            collect_tag_match(
                m,
                lang_name,
                source,
                &mut exported_names,
                &mut call_sites,
                &mut db_models,
                &mut external_calls,
                &mut const_strings,
                &mut launch_calls,
                &is_external_callee,
            );
        }
        if profile_queries {
            let process_secs = process_started
                .map(|started| started.elapsed().as_secs_f64())
                .unwrap_or(0.0);
            let wall_secs = wall_started
                .map(|started| started.elapsed().as_secs_f64())
                .unwrap_or(profile.elapsed_secs + process_secs);
            record_query_profile(lang_name, query_label, file_path, profile, process_secs, wall_secs, source_kind);
            if profile_query_lines && process_secs >= 0.010 {
                eprintln!(
                    "[ts-pack-index:query-process] lang={lang_name} label={} file={} matches={} process_ms={:.2}",
                    query_label,
                    file_path,
                    profile.match_count,
                    process_secs * 1000.0,
                );
            }
            if profile_query_lines && wall_secs >= 0.010 && (wall_secs - profile.elapsed_secs - process_secs) >= 0.005 {
                eprintln!(
                    "[ts-pack-index:query-overhead] lang={lang_name} label={} file={} source_kind={} matches={} query_ms={:.2} process_ms={:.2} wall_ms={:.2}",
                    query_label,
                    file_path,
                    match source_kind {
                        QuerySourceKind::Prepared => "prepared",
                        QuerySourceKind::QueryText => "text",
                    },
                    profile.match_count,
                    profile.elapsed_secs * 1000.0,
                    process_secs * 1000.0,
                    wall_secs * 1000.0,
                );
            }
        }
    }
    if !saw_match && lang_name != "python" {
        return None;
    }

    if lang_name == "python" {
        let extra = resolve_python_launch_idents(tree, source);
        if !extra.is_empty() {
            let mut seen: HashSet<String> = launch_calls.iter().cloned().collect();
            for item in extra {
                if seen.insert(item.clone()) {
                    launch_calls.push(item);
                }
            }
        }
    }

    Some(TagsResult {
        exported_names,
        call_sites,
        db_models,
        external_calls,
        const_strings,
        launch_calls,
    })
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn tags_query(lang: &str) -> Option<&'static str> {
    match lang {
        "rust" => Some(RUST_TAGS),
        "python" => Some(PYTHON_TAGS),
        "javascript" => Some(JS_CALL_TAGS),
        "typescript" | "tsx" => Some(TS_CALL_TAGS),
        "go" => Some(GO_TAGS),
        "swift" => Some(SWIFT_TAGS),
        _ => None,
    }
}

fn valid_tags_query(cache_key: &str, lang: &str, raw_query: &'static str) -> Option<Arc<String>> {
    let started = Instant::now();
    if let Some(query) = VALID_TAGS_QUERY_CACHE
        .read()
        .ok()
        .and_then(|cache| cache.get(cache_key).cloned())
    {
        if let Ok(mut agg) = VALID_TAGS_QUERY_PROFILE.lock() {
            agg.entry((lang.to_string(), cache_key.to_string()))
                .or_default()
                .record(true, started.elapsed().as_secs_f64());
        }
        return Some(query);
    }

    let valid_patterns: Vec<String> = split_query_patterns(raw_query)
        .into_iter()
        .filter(|pattern| ts_pack::query_compiles(lang, pattern))
        .collect();

    if valid_patterns.is_empty() {
        return None;
    }

    let combined = Arc::new(valid_patterns.join("\n\n"));
    let result = if let Ok(mut cache) = VALID_TAGS_QUERY_CACHE.write() {
        let entry = cache
            .entry(cache_key.to_string())
            .or_insert_with(|| Arc::clone(&combined));
        Some(Arc::clone(entry))
    } else {
        Some(combined)
    };
    if let Ok(mut agg) = VALID_TAGS_QUERY_PROFILE.lock() {
        agg.entry((lang.to_string(), cache_key.to_string()))
            .or_default()
            .record(false, started.elapsed().as_secs_f64());
    }
    result
}

pub(crate) fn build_js_ts_query_bundles() -> BatchTagQueryBundles {
    for lang in ["javascript", "typescript"] {
        if !ts_pack::has_language(lang) {
            let _ = ts_pack::download(&[lang]);
        }
    }
    BatchTagQueryBundles {
        typescript: build_fixed_js_ts_bundle("typescript", true),
        javascript: build_fixed_js_ts_bundle("javascript", false),
    }
}

fn query_sources_for(lang: &str, source: &[u8]) -> Option<Vec<(String, TagQuerySource)>> {
    match lang {
        "javascript" => js_ts_query_sources("javascript", false, source),
        "typescript" | "tsx" => js_ts_query_sources(lang, true, source),
        _ => {
            tags_query(lang).and_then(|query| {
                valid_tags_query(lang, lang, query).map(|q| vec![(lang.to_string(), TagQuerySource::QueryText(q))])
            })
        }
    }
}

fn build_fixed_js_ts_bundle(lang: &str, is_typescript: bool) -> Option<TagQueryBundle> {
    let call_key = if is_typescript {
        "typescript:call"
    } else {
        "javascript:call"
    };
    let call_raw = if is_typescript { TS_CALL_TAGS } else { JS_CALL_TAGS };
    let mut queries = vec![(
        call_key.to_string(),
        TagQuerySource::Prepared(prepare_valid_tags_query(lang, call_raw)?),
    )];
    if let Some(query) = prepare_valid_tags_query(lang, JS_TS_DB_TAGS)
    {
        queries.push(("js-ts:db".to_string(), TagQuerySource::Prepared(query)));
    }
    if let Some(query) = prepare_valid_tags_query(lang, JS_TS_EXTERNAL_TAGS)
    {
        queries.push(("js-ts:external".to_string(), TagQuerySource::Prepared(query)));
    }
    if let Some(query) = prepare_valid_tags_query(lang, JS_TS_CONST_TAGS)
    {
        queries.push(("js-ts:const".to_string(), TagQuerySource::Prepared(query)));
    }
    Some(TagQueryBundle::from_queries(queries))
}

fn js_ts_query_sources(lang: &str, is_typescript: bool, source: &[u8]) -> Option<Vec<(String, TagQuerySource)>> {
    let call_key = if is_typescript {
        "typescript:call"
    } else {
        "javascript:call"
    };
    let call_raw = if is_typescript { TS_CALL_TAGS } else { JS_CALL_TAGS };
    let mut queries = vec![(
        call_key.to_string(),
        TagQuerySource::QueryText(valid_tags_query(call_key, lang, call_raw)?),
    )];
    let source_text = std::str::from_utf8(source).ok().unwrap_or("");
    let wants_db = source_text.contains("prisma")
        || source_text.contains("prismaClient")
        || source_text.contains("tx.")
        || source_text.contains("db.");
    let wants_external = has_js_ts_external_call_hints(source_text);
    let wants_const = wants_external || source_text.contains("process.env") || source_text.contains("import.meta.env");

    if wants_db {
        if let Some(query) = valid_tags_query("js-ts:db", lang, JS_TS_DB_TAGS) {
            queries.push(("js-ts:db".to_string(), TagQuerySource::QueryText(query)));
        }
    }

    if wants_external {
        if let Some(query) = valid_tags_query("js-ts:external", lang, JS_TS_EXTERNAL_TAGS) {
            queries.push(("js-ts:external".to_string(), TagQuerySource::QueryText(query)));
        }
    }
    if wants_const {
        if let Some(query) = valid_tags_query("js-ts:const", lang, JS_TS_CONST_TAGS) {
            queries.push(("js-ts:const".to_string(), TagQuerySource::QueryText(query)));
        }
    }

    Some(queries)
}

fn prepare_valid_tags_query(lang: &str, raw_query: &'static str) -> Option<ts_pack::PreparedQuery> {
    let valid_patterns: Vec<String> = split_query_patterns(raw_query)
        .into_iter()
        .filter(|pattern| ts_pack::query_compiles(lang, pattern))
        .collect();
    if valid_patterns.is_empty() {
        return None;
    }
    let combined = valid_patterns.join("\n\n");
    ts_pack::prepare_query(lang, &combined).ok()
}

fn filter_js_ts_bundle(bundle: &TagQueryBundle, source: &[u8]) -> TagQueryBundle {
    let source_text = std::str::from_utf8(source).ok().unwrap_or("");
    let wants_db = source_text.contains("prisma")
        || source_text.contains("prismaClient")
        || source_text.contains("tx.")
        || source_text.contains("db.");
    let wants_external = has_js_ts_external_call_hints(source_text);
    let wants_const = wants_external || source_text.contains("process.env") || source_text.contains("import.meta.env");

    let queries = bundle
        .as_slice()
        .iter()
        .filter(|(label, _)| match label.as_str() {
            "js-ts:db" => wants_db,
            "js-ts:external" => wants_external,
            "js-ts:const" => wants_const,
            _ => true,
        })
        .cloned()
        .collect();
    TagQueryBundle::from_queries(queries)
}

fn has_js_ts_external_call_hints(source_text: &str) -> bool {
    source_text.contains("fetch(")
        || source_text.contains("axios(")
        || source_text.contains("ofetch(")
        || source_text.contains("$fetch(")
        || source_text.contains("ky(")
        || source_text.contains("new URL(")
}

fn external_query_ranges(source: &[u8]) -> Option<Vec<Range<usize>>> {
    let source_text = std::str::from_utf8(source).ok()?;
    const HINTS: &[&str] = &["fetch(", "axios(", "ofetch(", "$fetch(", "ky(", "new URL("];
    let mut ranges: Vec<Range<usize>> = HINTS
        .iter()
        .flat_map(|hint| source_text.match_indices(hint).map(|(idx, _)| idx))
        .map(|idx| {
            let start = idx.saturating_sub(256);
            let end = (idx + 4096).min(source.len());
            start..end
        })
        .collect();
    if ranges.is_empty() {
        return None;
    }
    ranges.sort_by_key(|range| range.start);
    let mut merged: Vec<Range<usize>> = Vec::with_capacity(ranges.len());
    for range in ranges {
        if let Some(last) = merged.last_mut() {
            if range.start <= last.end {
                last.end = last.end.max(range.end);
                continue;
            }
        }
        merged.push(range);
    }
    Some(merged)
}

fn run_query_with_optional_ranges(
    tree: &ts_pack::Tree,
    language: &str,
    query_source: &str,
    source: &[u8],
    ranges: Option<&[Range<usize>]>,
) -> Result<Vec<ts_pack::QueryMatch>, ts_pack::Error> {
    let Some(ranges) = ranges else {
        return ts_pack::run_query(tree, language, query_source, source);
    };
    let mut out = Vec::new();
    for range in ranges {
        out.extend(ts_pack::run_query_in_byte_range(
            tree,
            language,
            query_source,
            source,
            range.clone(),
        )?);
    }
    Ok(out)
}

fn run_query_profiled_with_optional_ranges(
    tree: &ts_pack::Tree,
    language: &str,
    query_source: &str,
    source: &[u8],
    ranges: Option<&[Range<usize>]>,
) -> Result<(Vec<ts_pack::QueryMatch>, ts_pack::QueryProfile), ts_pack::Error> {
    let Some(ranges) = ranges else {
        return ts_pack::run_query_profiled(tree, language, query_source, source);
    };
    let mut out = Vec::new();
    let mut profile = ts_pack::QueryProfile::default();
    for range in ranges {
        let (matches, current) = ts_pack::run_query_in_byte_range_profiled(
            tree,
            language,
            query_source,
            source,
            range.clone(),
        )?;
        out.extend(matches);
        profile.lookup_secs += current.lookup_secs;
        profile.match_count += current.match_count;
        profile.exceeded_match_limit |= current.exceeded_match_limit;
        profile.used_byte_range = true;
        profile.elapsed_secs += current.elapsed_secs;
    }
    Ok((out, profile))
}

fn run_prepared_query_with_optional_ranges(
    tree: &ts_pack::Tree,
    prepared: &ts_pack::PreparedQuery,
    source: &[u8],
    ranges: Option<&[Range<usize>]>,
) -> Result<Vec<ts_pack::QueryMatch>, ts_pack::Error> {
    let Some(ranges) = ranges else {
        return ts_pack::run_prepared_query(tree, prepared, source);
    };
    let mut out = Vec::new();
    for range in ranges {
        out.extend(ts_pack::run_prepared_query_in_byte_range(
            tree,
            prepared,
            source,
            range.clone(),
        )?);
    }
    Ok(out)
}

fn run_prepared_query_profiled_with_optional_ranges(
    tree: &ts_pack::Tree,
    prepared: &ts_pack::PreparedQuery,
    source: &[u8],
    ranges: Option<&[Range<usize>]>,
) -> Result<(Vec<ts_pack::QueryMatch>, ts_pack::QueryProfile), ts_pack::Error> {
    let Some(ranges) = ranges else {
        return ts_pack::run_prepared_query_profiled(tree, prepared, source);
    };
    let mut out = Vec::new();
    let mut profile = ts_pack::QueryProfile::default();
    for range in ranges {
        let (matches, current) =
            ts_pack::run_prepared_query_in_byte_range_profiled(tree, prepared, source, range.clone())?;
        out.extend(matches);
        profile.match_count += current.match_count;
        profile.exceeded_match_limit |= current.exceeded_match_limit;
        profile.used_byte_range = true;
        profile.elapsed_secs += current.elapsed_secs;
    }
    Ok((out, profile))
}

fn collect_tag_match(
    m: &ts_pack::QueryMatch,
    lang_name: &str,
    source: &[u8],
    exported_names: &mut HashSet<String>,
    call_sites: &mut Vec<CallSite>,
    db_models: &mut HashSet<String>,
    external_calls: &mut Vec<ExternalCallSite>,
    const_strings: &mut HashMap<String, String>,
    launch_calls: &mut Vec<String>,
    is_external_callee: &dyn Fn(&str) -> bool,
) {
    let capture_text = |node_info: &ts_pack::NodeInfo| -> Option<&str> {
        std::str::from_utf8(&source[node_info.start_byte..node_info.end_byte]).ok()
    };
    let is_export_pattern = m.captures.iter().any(|(cap, _)| cap == "exported");
    let mut has_vis = false;
    let mut def_name: Option<String> = None;
    let mut callee_site: Option<(usize, String)> = None;
    let mut receiver_name: Option<String> = None;
    let mut external_callee: Option<String> = None;
    let mut external_arg: Option<ExternalCallArg> = None;
    let mut external_arg_left: Option<String> = None;
    let mut external_arg_right: Option<String> = None;
    let mut external_arg_left_is_literal = false;
    let mut external_arg_right_is_literal = false;
    let mut const_name: Option<String> = None;
    let mut const_value: Option<String> = None;
    let mut const_left: Option<String> = None;
    let mut const_right: Option<String> = None;
    let mut external_url_ctor: Option<String> = None;
    let mut external_url_path: Option<String> = None;
    let mut external_url_base: Option<String> = None;
    let mut external_url_base_ident: Option<String> = None;
    let mut env_root: Option<String> = None;
    let mut env_prop: Option<String> = None;
    let mut env_import: Option<String> = None;
    let mut env_meta: Option<String> = None;
    let mut env_env: Option<String> = None;
    let mut env_key: Option<String> = None;
    let mut launch_module: Option<String> = None;
    let mut launch_callee: Option<String> = None;
    let mut launch_args: Vec<String> = Vec::new();

    for (cap_name, node_info) in &m.captures {
        match cap_name.as_ref() {
            "vis" => {
                let Some(text) = capture_text(node_info) else {
                    continue;
                };
                if lang_name == "swift" {
                    let lowered = text.to_lowercase();
                    if lowered.contains("public") || lowered.contains("open") {
                        has_vis = true;
                    }
                } else {
                    has_vis = true;
                }
            }
            "name" => {
                if let Some(text) = capture_text(node_info) {
                    def_name = Some(text.to_string());
                }
            }
            "callee" => {
                if let Some(text) = capture_text(node_info) {
                    callee_site = Some((node_info.start_byte, text.to_string()));
                }
            }
            "recv" => {
                if let Some(text) = capture_text(node_info) {
                    receiver_name = Some(text.to_string());
                }
            }
            "db" => {
                if source.get(node_info.start_byte).copied() != Some(b'$')
                    && is_delegate_property_use(source, node_info.end_byte)
                {
                    if let Some(text) = capture_text(node_info) {
                        db_models.insert(text.to_string());
                    }
                }
            }
            "external_callee" => {
                if let Some(text) = capture_text(node_info) {
                    external_callee = Some(text.to_string());
                }
            }
            "external_arg" => {
                let Some(text) = capture_text(node_info) else {
                    continue;
                };
                if let Some(literal) = strip_string_literal(&text) {
                    external_arg = Some(ExternalCallArg::Literal(literal));
                } else if text.starts_with('`') && text.contains("${") {
                    external_arg = parse_simple_template_arg(text);
                } else {
                    external_arg = Some(ExternalCallArg::Identifier(text.to_string()));
                }
            }
            "external_arg_left" => {
                let Some(text) = capture_text(node_info) else {
                    continue;
                };
                if let Some(literal) = strip_string_literal(&text) {
                    external_arg_left = Some(literal);
                    external_arg_left_is_literal = true;
                } else {
                    external_arg_left = Some(text.to_string());
                    external_arg_left_is_literal = false;
                }
            }
            "external_arg_right" => {
                let Some(text) = capture_text(node_info) else {
                    continue;
                };
                if let Some(literal) = strip_string_literal(&text) {
                    external_arg_right = Some(literal);
                    external_arg_right_is_literal = true;
                } else {
                    external_arg_right = Some(text.to_string());
                    external_arg_right_is_literal = false;
                }
            }
            "const_name" => {
                if let Some(text) = capture_text(node_info) {
                    const_name = Some(text.to_string());
                }
            }
            "const_value" => {
                if let Some(text) = capture_text(node_info) {
                    const_value = strip_string_literal(text);
                }
            }
            "const_left" => {
                if let Some(text) = capture_text(node_info) {
                    const_left = strip_string_literal(text);
                }
            }
            "const_right" => {
                if let Some(text) = capture_text(node_info) {
                    const_right = strip_string_literal(text);
                }
            }
            "external_url_ctor" => {
                if let Some(text) = capture_text(node_info) {
                    external_url_ctor = Some(text.to_string());
                }
            }
            "external_url_path" => {
                if let Some(text) = capture_text(node_info) {
                    external_url_path = strip_string_literal(text);
                }
            }
            "external_url_base" => {
                if let Some(text) = capture_text(node_info) {
                    external_url_base = strip_string_literal(text);
                }
            }
            "external_url_base_ident" => {
                if let Some(text) = capture_text(node_info) {
                    external_url_base_ident = Some(text.to_string());
                }
            }
            "env_root" => {
                if let Some(text) = capture_text(node_info) {
                    env_root = Some(text.to_string());
                }
            }
            "env_prop" => {
                if let Some(text) = capture_text(node_info) {
                    env_prop = Some(text.to_string());
                }
            }
            "env_import" => {
                if let Some(text) = capture_text(node_info) {
                    env_import = Some(text.to_string());
                }
            }
            "env_meta" => {
                if let Some(text) = capture_text(node_info) {
                    env_meta = Some(text.to_string());
                }
            }
            "env_env" => {
                if let Some(text) = capture_text(node_info) {
                    env_env = Some(text.to_string());
                }
            }
            "env_key" => {
                if let Some(text) = capture_text(node_info) {
                    env_key = Some(text.to_string());
                }
            }
            "launch_module" => {
                if let Some(text) = capture_text(node_info) {
                    launch_module = Some(text.to_string());
                }
            }
            "launch_callee" => {
                if let Some(text) = capture_text(node_info) {
                    launch_callee = Some(text.to_string());
                }
            }
            "launch_arg" => {
                let Some(text) = capture_text(node_info) else {
                    continue;
                };
                if let Some(literal) = strip_string_literal(&text) {
                    launch_args.push(literal);
                }
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
        call_sites.push(CallSite {
            start_byte,
            callee,
            receiver: receiver_name,
        });
    }

    if let Some(name) = const_name {
        if let Some(value) = const_value {
            const_strings.insert(name, value);
        } else if let (Some(left), Some(right)) = (const_left, const_right) {
            const_strings.insert(name, format!("{left}{right}"));
        } else if let (Some(key), Some(root), Some(prop)) = (env_key.clone(), env_root, env_prop) {
            if root == "process" && prop == "env" {
                const_strings.insert(name, format!("env://{key}"));
            }
        } else if let (Some(key), Some(import), Some(meta), Some(env)) =
            (env_key.clone(), env_import, env_meta, env_env)
        {
            if import == "import" && meta == "meta" && env == "env" {
                const_strings.insert(name, format!("env://{key}"));
            }
        }
    }

    if let (Some(callee), Some(arg)) = (external_callee.as_ref(), external_arg) {
        if is_external_callee(callee.as_str()) {
            external_calls.push(ExternalCallSite { arg });
        }
    } else if let (Some(callee), Some(left), Some(right)) =
        (external_callee.as_ref(), external_arg_left, external_arg_right)
    {
        if is_external_callee(callee.as_str()) {
            if external_arg_left_is_literal && !external_arg_right_is_literal {
                external_calls.push(ExternalCallSite {
                    arg: ExternalCallArg::ConcatLiteralIdent {
                        literal: left,
                        ident: right,
                    },
                });
            } else if !external_arg_left_is_literal && external_arg_right_is_literal {
                external_calls.push(ExternalCallSite {
                    arg: ExternalCallArg::ConcatIdentLiteral {
                        ident: left,
                        literal: right,
                    },
                });
            } else if external_arg_left_is_literal && external_arg_right_is_literal {
                external_calls.push(ExternalCallSite {
                    arg: ExternalCallArg::Literal(format!("{left}{right}")),
                });
            }
        }
    } else if let (Some(callee), Some(ctor), Some(path)) = (external_callee, external_url_ctor, external_url_path) {
        if is_external_callee(callee.as_str()) && ctor == "URL" {
            if let Some(base) = external_url_base {
                external_calls.push(ExternalCallSite {
                    arg: ExternalCallArg::UrlLiteral { path, base },
                });
            } else if let Some(base_ident) = external_url_base_ident {
                external_calls.push(ExternalCallSite {
                    arg: ExternalCallArg::UrlWithBaseIdent { path, base_ident },
                });
            }
        }
    }

    if let (Some(module), Some(callee)) = (launch_module.as_ref(), launch_callee.as_ref()) {
        if is_launch_callee(module, callee.as_str()) {
            for arg in &launch_args {
                if arg.ends_with(".py") {
                    launch_calls.push(arg.clone());
                }
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn maybe_parse(lang: &str, source: &str) -> Option<ts_pack::Tree> {
        if !ts_pack::has_language(lang) {
            return None;
        }
        ts_pack::parse_string(lang, source.as_bytes()).ok()
    }

    #[test]
    fn extracts_javascript_external_calls_and_consts() {
        let source = r#"
        const API_BASE = "https://api.example.com";
        export const loadData = () => fetch(API_BASE + "/v1/items");
        const other = axios(new URL("/v2/stats", API_BASE));
        "#;
        let Some(tree) = maybe_parse("javascript", source) else {
            return;
        };
        let tags = run_tags("javascript", &tree, source.as_bytes(), "fixture.js", None).expect("tags");

        assert!(tags.exported_names.contains("loadData"));
        assert_eq!(
            tags.const_strings.get("API_BASE"),
            Some(&"https://api.example.com".to_string())
        );
        assert_eq!(tags.external_calls.len(), 2);
        assert!(matches!(
            &tags.external_calls[0].arg,
            ExternalCallArg::ConcatIdentLiteral { ident, literal }
                if ident == "API_BASE" && literal == "/v1/items"
        ));
        assert!(matches!(
            &tags.external_calls[1].arg,
            ExternalCallArg::UrlWithBaseIdent { path, base_ident }
                if path == "/v2/stats" && base_ident == "API_BASE"
        ));
    }

    #[test]
    fn extracts_typescript_external_calls_from_simple_template_strings() {
        let source = r#"
        const API_BASE = "https://api.example.com";
        export async function loadData() {
          return fetch(`${API_BASE}/v1/items`);
        }
        "#;
        let Some(tree) = maybe_parse("typescript", source) else {
            return;
        };
        let tags = run_tags("typescript", &tree, source.as_bytes(), "fixture.ts", None).expect("tags");

        assert_eq!(tags.external_calls.len(), 1);
        assert!(matches!(
            &tags.external_calls[0].arg,
            ExternalCallArg::ConcatIdentLiteral { ident, literal }
                if ident == "API_BASE" && literal == "/v1/items"
        ));
    }

    #[test]
    fn builds_prepared_js_ts_batch_bundles() {
        let bundles = build_js_ts_query_bundles();
        assert!(bundles.javascript.is_some(), "expected javascript bundle");
        assert!(bundles.typescript.is_some(), "expected typescript bundle");
    }

    #[test]
    fn extracts_python_launch_calls_from_literals_and_ident_lists() {
        let source = r#"
        import subprocess

        CMD = ["python", "scripts/worker.py"]
        subprocess.Popen(["python", "scripts/direct.py"])
        subprocess.run(CMD)
        "#;
        let Some(tree) = maybe_parse("python", source) else {
            return;
        };
        let tags = run_tags("python", &tree, source.as_bytes(), "fixture.py", None).expect("tags");

        assert!(tags.launch_calls.contains(&"scripts/direct.py".to_string()));
        assert!(tags.launch_calls.contains(&"scripts/worker.py".to_string()));
    }

    #[test]
    fn extracts_swift_receivers_for_navigation_calls() {
        let source = r#"
        public struct Service {
            func run() {
                self.worker.start()
            }
        }
        "#;
        let Some(tree) = maybe_parse("swift", source) else {
            return;
        };
        let tags = run_tags("swift", &tree, source.as_bytes(), "fixture.swift", None).expect("tags");

        assert!(
            tags.call_sites
                .iter()
                .any(|c| c.callee == "start" && c.receiver.as_deref() == Some("self"))
        );
    }

    #[test]
    fn extracts_typescript_prisma_and_tx_db_models() {
        let source = r#"
        export class QuickBooksService {
            async sync() {
                await this.prisma.accountingSyncConnection.findFirst({});
                await this.prisma.$transaction(async (tx) => {
                    await tx.accountingExternalAccount.deleteMany({});
                    await tx.accountingExternalAccount.createMany({});
                });
            }
        }
        "#;
        let Some(tree) = maybe_parse("typescript", source) else {
            return;
        };
        let tags = run_tags("typescript", &tree, source.as_bytes(), "fixture.ts", None).expect("tags");

        assert!(tags.db_models.contains("accountingSyncConnection"));
        assert!(tags.db_models.contains("accountingExternalAccount"));
    }

    #[test]
    fn extracts_typescript_direct_prisma_delegate_models() {
        let source = r#"
        export async function run(prismaClient: PrismaClient) {
            return prismaClient.tenantCredit.findMany({});
        }
        "#;
        let Some(tree) = maybe_parse("typescript", source) else {
            return;
        };
        let tags = run_tags("typescript", &tree, source.as_bytes(), "fixture.ts", None).expect("tags");

        assert!(tags.db_models.contains("tenantCredit"));
    }

    #[test]
    fn extracts_rental_history_service_models() {
        let source = r#"
        export class AccountingSyncHistoryService {
          constructor(private prisma: PrismaClient) {}

          async listQuickBooksAttempts(where: Prisma.AccountingJournalSyncAttemptWhereInput) {
            const items = await this.prisma.accountingJournalSyncAttempt.findMany({ where });
            const totalCount = await this.prisma.accountingJournalSyncAttempt.count({ where });
            return { items, totalCount };
          }

          async listQuickBooksNeedsAttention(where: Prisma.AccountingJournalSyncWhereInput) {
            const items = await this.prisma.accountingJournalSync.findMany({ where });
            const totalCount = await this.prisma.accountingJournalSync.count({ where });
            const failedCount = await this.prisma.accountingJournalSync.count({ where });
            return { items, totalCount, failedCount };
          }
        }
        "#;
        let Some(tree) = maybe_parse("typescript", source) else {
            return;
        };
        let tags = run_tags("typescript", &tree, source.as_bytes(), "fixture.ts", None).expect("tags");

        assert!(tags.db_models.contains("accountingJournalSyncAttempt"));
        assert!(tags.db_models.contains("accountingJournalSync"));
    }

    #[test]
    fn extracts_rental_credit_service_delegate_models_but_not_query_raw() {
        let source = r#"
        export class TenantCreditService {
          constructor(private prisma: PrismaClient) {}

          async createOverpaymentCredit(params: { amount: Prisma.Decimal }) {
            return this.prisma.tenantCredit.create({ data: params });
          }

          async getUnappliedCreditBalance(orgId: string, tenantId: string) {
            const rows = await this.prisma.$queryRaw<Array<{ total: Prisma.Decimal }>>`
              SELECT COALESCE(SUM(unapplied_amount), 0) AS total
              FROM tenant_credits
              WHERE org_id = ${orgId}
                AND tenant_id = ${tenantId}
            `;
            return rows[0]?.total;
          }

          async applyAvailableCreditsForTenant(orgId: string, tenantId: string) {
            const credits = await this.prisma.tenantCredit.findMany({ where: { orgId, tenantId } });
            const application = await this.prisma.tenantCreditApplication.create({
              data: { orgId, tenantId, amount: 1 },
            });
            await this.prisma.tenantCredit.update({
              where: { id: application.id },
              data: { memo: "applied" },
            });
            return credits;
          }
        }
        "#;
        let Some(tree) = maybe_parse("typescript", source) else {
            return;
        };
        let tags = run_tags("typescript", &tree, source.as_bytes(), "fixture.ts", None).expect("tags");

        assert!(tags.db_models.contains("tenantCredit"));
        assert!(tags.db_models.contains("tenantCreditApplication"));
        assert!(!tags.db_models.contains("$queryRaw"));
    }
}
#[derive(Debug, Clone, Copy, Default)]
struct ValidTagsQueryAggregate {
    hits: usize,
    misses: usize,
    total_secs: f64,
    max_secs: f64,
}

impl ValidTagsQueryAggregate {
    fn record(&mut self, hit: bool, elapsed_secs: f64) {
        if hit {
            self.hits += 1;
        } else {
            self.misses += 1;
        }
        self.total_secs += elapsed_secs;
        self.max_secs = self.max_secs.max(elapsed_secs);
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ValidTagsQuerySummaryRow {
    pub(crate) lang: String,
    pub(crate) cache_key: String,
    pub(crate) hits: usize,
    pub(crate) misses: usize,
    pub(crate) total_secs: f64,
    pub(crate) max_secs: f64,
}
