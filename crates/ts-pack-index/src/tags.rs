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

(member_expression
  object: (identifier) @dbobj
  property: (property_identifier) @db)
(#eq? @dbobj "prisma")

(member_expression
  object: (identifier) @dbobj
  property: (identifier) @db)
(#eq? @dbobj "prisma")

(member_expression
  object: (identifier) @dbobj
  property: (property_identifier) @db)
(#eq? @dbobj "tx")

(member_expression
  object: (identifier) @dbobj
  property: (identifier) @db)
(#eq? @dbobj "tx")

(member_expression
  object: (identifier) @dbobj
  property: (property_identifier) @db)
(#eq? @dbobj "prismaClient")

(member_expression
  object: (identifier) @dbobj
  property: (identifier) @db)
(#eq? @dbobj "prismaClient")

(member_expression
  object: (identifier) @dbobj
  property: (property_identifier) @db)
(#eq? @dbobj "db")

(member_expression
  object: (identifier) @dbobj
  property: (identifier) @db)
(#eq? @dbobj "db")

(member_expression
  object: (member_expression
    object: (this)
    property: (property_identifier) @dbobj)
  property: (property_identifier) @db)
(#eq? @dbobj "prisma")

(member_expression
  object: (member_expression
    object: (this)
    property: (identifier) @dbobj)
  property: (identifier) @db)
(#eq? @dbobj "prisma")

(member_expression
  object: (member_expression
    property: (property_identifier) @dbobj)
  property: (property_identifier) @db)
(#eq? @dbobj "prisma")

(member_expression
  object: (member_expression
    property: (identifier) @dbobj)
  property: (identifier) @db)
(#eq? @dbobj "prisma")

(member_expression
  object: (member_expression
    object: (identifier) @ctx
    property: (property_identifier) @dbobj)
  property: (property_identifier) @db)
(#eq? @dbobj "prisma")

(member_expression
  object: (member_expression
    object: (identifier) @ctx
    property: (identifier) @dbobj)
  property: (identifier) @db)
(#eq? @dbobj "prisma")

(member_expression
  object: (member_expression
    object: (identifier) @ctx
    property: (property_identifier) @dbobj)
  property: (property_identifier) @db)
(#match? @dbobj ".*Prisma$")

(member_expression
  object: (member_expression
    object: (identifier) @ctx
    property: (identifier) @dbobj)
  property: (identifier) @db)
(#match? @dbobj ".*Prisma$")

(call_expression
  function: (identifier) @external_callee
  arguments: (arguments (string) @external_arg))

(call_expression
  function: (identifier) @external_callee
  arguments: (arguments (identifier) @external_arg))

(call_expression
  function: (identifier) @external_callee
  arguments: (arguments (template_string) @external_arg))

(call_expression
  function: (member_expression
    object: (identifier) @external_callee)
  arguments: (arguments (string) @external_arg))

(call_expression
  function: (member_expression
    object: (identifier) @external_callee)
  arguments: (arguments (identifier) @external_arg))

(call_expression
  function: (member_expression
    object: (identifier) @external_callee)
  arguments: (arguments (template_string) @external_arg))

(call_expression
  function: (identifier) @external_callee
  arguments: (arguments
    (binary_expression
      left: (identifier) @external_arg_left
      operator: "+"
      right: (string) @external_arg_right)))

(call_expression
  function: (identifier) @external_callee
  arguments: (arguments
    (binary_expression
      left: (string) @external_arg_left
      operator: "+"
      right: (identifier) @external_arg_right)))

(call_expression
  function: (member_expression
    object: (identifier) @external_callee)
  arguments: (arguments
    (binary_expression
      left: (identifier) @external_arg_left
      operator: "+"
      right: (string) @external_arg_right)))

(call_expression
  function: (member_expression
    object: (identifier) @external_callee)
  arguments: (arguments
    (binary_expression
      left: (string) @external_arg_left
      operator: "+"
      right: (identifier) @external_arg_right)))

(call_expression
  function: (identifier) @external_callee
  arguments: (arguments
    (new_expression
      constructor: (identifier) @external_url_ctor
      arguments: (arguments (string) @external_url_path (string) @external_url_base))))

(call_expression
  function: (identifier) @external_callee
  arguments: (arguments
    (new_expression
      constructor: (identifier) @external_url_ctor
      arguments: (arguments (string) @external_url_path (identifier) @external_url_base_ident))))

(call_expression
  function: (member_expression
    object: (identifier) @external_callee)
  arguments: (arguments
    (new_expression
      constructor: (identifier) @external_url_ctor
      arguments: (arguments (string) @external_url_path (string) @external_url_base))))

(call_expression
  function: (member_expression
    object: (identifier) @external_callee)
  arguments: (arguments
    (new_expression
      constructor: (identifier) @external_url_ctor
      arguments: (arguments (string) @external_url_path (identifier) @external_url_base_ident))))

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

(import_clause
  name: (identifier) @import_name)

(import_clause
  (named_imports
    (import_specifier
      name: (identifier) @import_named)))

(import_clause
  (named_imports
    (import_specifier
      name: (property_identifier) @import_named)))

(import_clause
  (namespace_import
    (identifier) @import_star))
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

(member_expression
  object: (identifier) @dbobj
  property: (property_identifier) @db)
(#eq? @dbobj "prisma")

(member_expression
  object: (identifier) @dbobj
  property: (identifier) @db)
(#eq? @dbobj "prisma")

(member_expression
  object: (identifier) @dbobj
  property: (property_identifier) @db)
(#eq? @dbobj "tx")

(member_expression
  object: (identifier) @dbobj
  property: (identifier) @db)
(#eq? @dbobj "tx")

(member_expression
  object: (identifier) @dbobj
  property: (property_identifier) @db)
(#eq? @dbobj "prismaClient")

(member_expression
  object: (identifier) @dbobj
  property: (identifier) @db)
(#eq? @dbobj "prismaClient")

(member_expression
  object: (identifier) @dbobj
  property: (property_identifier) @db)
(#eq? @dbobj "db")

(member_expression
  object: (identifier) @dbobj
  property: (identifier) @db)
(#eq? @dbobj "db")

(member_expression
  object: (member_expression
    object: (this)
    property: (property_identifier) @dbobj)
  property: (property_identifier) @db)
(#eq? @dbobj "prisma")

(member_expression
  object: (member_expression
    object: (this)
    property: (identifier) @dbobj)
  property: (identifier) @db)
(#eq? @dbobj "prisma")

(member_expression
  object: (member_expression
    property: (property_identifier) @dbobj)
  property: (property_identifier) @db)
(#eq? @dbobj "prisma")

(member_expression
  object: (member_expression
    property: (identifier) @dbobj)
  property: (identifier) @db)
(#eq? @dbobj "prisma")

(member_expression
  object: (member_expression
    object: (identifier) @ctx
    property: (property_identifier) @dbobj)
  property: (property_identifier) @db)
(#eq? @dbobj "prisma")

(member_expression
  object: (member_expression
    object: (identifier) @ctx
    property: (identifier) @dbobj)
  property: (identifier) @db)
(#eq? @dbobj "prisma")

(member_expression
  object: (member_expression
    object: (identifier) @ctx
    property: (property_identifier) @dbobj)
  property: (property_identifier) @db)
(#match? @dbobj ".*Prisma$")

(member_expression
  object: (member_expression
    object: (identifier) @ctx
    property: (identifier) @dbobj)
  property: (identifier) @db)
(#match? @dbobj ".*Prisma$")

(call_expression
  function: (identifier) @external_callee
  arguments: (arguments (string) @external_arg))

(call_expression
  function: (identifier) @external_callee
  arguments: (arguments (identifier) @external_arg))

(call_expression
  function: (identifier) @external_callee
  arguments: (arguments (template_string) @external_arg))

(call_expression
  function: (member_expression
    object: (identifier) @external_callee)
  arguments: (arguments (string) @external_arg))

(call_expression
  function: (member_expression
    object: (identifier) @external_callee)
  arguments: (arguments (identifier) @external_arg))

(call_expression
  function: (member_expression
    object: (identifier) @external_callee)
  arguments: (arguments (template_string) @external_arg))

(call_expression
  function: (identifier) @external_callee
  arguments: (arguments
    (binary_expression
      left: (identifier) @external_arg_left
      operator: "+"
      right: (string) @external_arg_right)))

(call_expression
  function: (identifier) @external_callee
  arguments: (arguments
    (binary_expression
      left: (string) @external_arg_left
      operator: "+"
      right: (identifier) @external_arg_right)))

(call_expression
  function: (member_expression
    object: (identifier) @external_callee)
  arguments: (arguments
    (binary_expression
      left: (identifier) @external_arg_left
      operator: "+"
      right: (string) @external_arg_right)))

(call_expression
  function: (member_expression
    object: (identifier) @external_callee)
  arguments: (arguments
    (binary_expression
      left: (string) @external_arg_left
      operator: "+"
      right: (identifier) @external_arg_right)))

(call_expression
  function: (identifier) @external_callee
  arguments: (arguments
    (new_expression
      constructor: (identifier) @external_url_ctor
      arguments: (arguments (string) @external_url_path (string) @external_url_base))))

(call_expression
  function: (identifier) @external_callee
  arguments: (arguments
    (new_expression
      constructor: (identifier) @external_url_ctor
      arguments: (arguments (string) @external_url_path (identifier) @external_url_base_ident))))

(call_expression
  function: (member_expression
    object: (identifier) @external_callee)
  arguments: (arguments
    (new_expression
      constructor: (identifier) @external_url_ctor
      arguments: (arguments (string) @external_url_path (string) @external_url_base))))

(call_expression
  function: (member_expression
    object: (identifier) @external_callee)
  arguments: (arguments
    (new_expression
      constructor: (identifier) @external_url_ctor
      arguments: (arguments (string) @external_url_path (identifier) @external_url_base_ident))))

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

(import_clause
  name: (identifier) @import_name)

(import_clause
  (named_imports
    (import_specifier
      name: (identifier) @import_named)))

(import_clause
  (named_imports
    (import_specifier
      name: (property_identifier) @import_named)))

(import_clause
  (namespace_import
    (identifier) @import_star))
"#;

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
    /// Receiver identifier for member calls (Swift only).
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
    /// Prisma delegate accesses (ts/js only).
    pub db_delegates: std::collections::HashSet<String>,
    /// External API call sites (js/ts only).
    pub external_calls: Vec<ExternalCallSite>,
    /// Constant string assignments (js/ts only).
    pub const_strings: std::collections::HashMap<String, String>,
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
    let mut db_delegates = std::collections::HashSet::new();
    let mut external_calls = Vec::new();
    let mut const_strings = std::collections::HashMap::new();

    let is_external_callee = |name: &str| matches!(name, "fetch" | "axios" | "ky" | "ofetch" | "$fetch");

    let strip_string_literal = |raw: &str| {
        let trimmed = raw.trim();
        if trimmed.len() < 2 {
            return None;
        }
        let first = trimmed.chars().next()?;
        let last = trimmed.chars().last()?;
        let is_quote =
            (first == '"' && last == '"') || (first == '\'' && last == '\'') || (first == '`' && last == '`');
        if !is_quote {
            return None;
        }
        let inner = &trimmed[1..trimmed.len() - 1];
        if inner.contains("${") {
            return None;
        }
        Some(inner.to_string())
    };

    for pattern in &patterns {
        let matches = match ts_pack::run_query(tree, lang_name, pattern, source) {
            Ok(m) => m,
            Err(_) => continue, // invalid node type for this grammar — skip
        };

        for m in &matches {
            let is_export_pattern = m.captures.iter().any(|(cap, _)| cap == "exported");
            let mut has_vis = false;
            let mut def_name: Option<String> = None;
            // (start_byte, callee_name, receiver)
            let mut callee_site: Option<(usize, String, Option<String>)> = None;
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

            for (cap_name, node_info) in &m.captures {
                let text = match ts_pack::extract_text(source, node_info) {
                    Ok(t) => t.to_string(),
                    Err(_) => continue,
                };

                match cap_name.as_ref() {
                    "vis" => {
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
                        def_name = Some(text);
                    }
                    "callee" => {
                        callee_site = Some((node_info.start_byte, text, receiver_name.clone()));
                    }
                    "recv" => {
                        receiver_name = Some(text);
                    }
                    "db" => {
                        db_delegates.insert(text);
                    }
                    "external_callee" => {
                        external_callee = Some(text);
                    }
                    "external_arg" => {
                        if let Some(literal) = strip_string_literal(&text) {
                            external_arg = Some(ExternalCallArg::Literal(literal));
                        } else if text.starts_with('`') && text.contains("${") {
                            continue;
                        } else {
                            external_arg = Some(ExternalCallArg::Identifier(text));
                        }
                    }
                    "external_arg_left" => {
                        if let Some(literal) = strip_string_literal(&text) {
                            external_arg_left = Some(literal);
                            external_arg_left_is_literal = true;
                        } else {
                            external_arg_left = Some(text);
                            external_arg_left_is_literal = false;
                        }
                    }
                    "external_arg_right" => {
                        if let Some(literal) = strip_string_literal(&text) {
                            external_arg_right = Some(literal);
                            external_arg_right_is_literal = true;
                        } else {
                            external_arg_right = Some(text);
                            external_arg_right_is_literal = false;
                        }
                    }
                    "const_name" => {
                        const_name = Some(text);
                    }
                    "const_value" => {
                        const_value = strip_string_literal(&text);
                    }
                    "const_left" => {
                        const_left = strip_string_literal(&text);
                    }
                    "const_right" => {
                        const_right = strip_string_literal(&text);
                    }
                    "external_url_ctor" => {
                        external_url_ctor = Some(text);
                    }
                    "external_url_path" => {
                        external_url_path = strip_string_literal(&text);
                    }
                    "external_url_base" => {
                        external_url_base = strip_string_literal(&text);
                    }
                    "external_url_base_ident" => {
                        external_url_base_ident = Some(text);
                    }
                    "env_root" => {
                        env_root = Some(text);
                    }
                    "env_prop" => {
                        env_prop = Some(text);
                    }
                    "env_import" => {
                        env_import = Some(text);
                    }
                    "env_meta" => {
                        env_meta = Some(text);
                    }
                    "env_env" => {
                        env_env = Some(text);
                    }
                    "env_key" => {
                        env_key = Some(text);
                    }
                    _ => {}
                }
            }

            if let Some(name) = def_name {
                if has_vis || is_export_pattern {
                    exported_names.insert(name);
                }
            }

            if let Some((start_byte, callee, receiver)) = callee_site {
                call_sites.push(CallSite {
                    start_byte,
                    callee,
                    receiver,
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
            } else if let (Some(callee), Some(ctor), Some(path)) =
                (external_callee, external_url_ctor, external_url_path)
            {
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
        }
    }

    Some(TagsResult {
        exported_names,
        call_sites,
        db_delegates,
        external_calls,
        const_strings,
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
