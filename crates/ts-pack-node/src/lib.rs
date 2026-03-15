use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Returns an array of all available language names.
#[napi(js_name = "availableLanguages")]
pub fn available_languages() -> Vec<String> {
    ts_pack_core::available_languages()
}

/// Checks whether a language with the given name is available.
#[napi(js_name = "hasLanguage")]
pub fn has_language(name: String) -> bool {
    ts_pack_core::has_language(&name)
}

/// Returns the number of available languages.
#[napi(js_name = "languageCount")]
pub fn language_count() -> u32 {
    ts_pack_core::language_count() as u32
}

/// Returns the raw TSLanguage pointer for interop with node-tree-sitter.
///
/// Throws an error if the language is not found.
#[napi(js_name = "getLanguagePtr")]
pub fn get_language_ptr(name: String) -> napi::Result<i64> {
    let language = ts_pack_core::get_language(&name).map_err(|e| napi::Error::from_reason(format!("{e}")))?;
    let ptr = language.into_raw() as i64;
    Ok(ptr)
}

// ---------------------------------------------------------------------------
// Parsing functions
// ---------------------------------------------------------------------------

/// Parse a source string using the named language and return an opaque tree handle.
///
/// Throws an error if the language is not found or parsing fails.
#[napi(js_name = "parseString")]
pub fn parse_string(language: String, source: String) -> napi::Result<External<tree_sitter::Tree>> {
    let tree = ts_pack_core::parse_string(&language, source.as_bytes())
        .map_err(|e| napi::Error::from_reason(format!("{e}")))?;
    Ok(External::new(tree))
}

/// Get the type name of the root node.
#[napi(js_name = "treeRootNodeType")]
pub fn tree_root_node_type(tree: &External<tree_sitter::Tree>) -> String {
    tree.root_node().kind().to_string()
}

/// Get the number of named children of the root node.
#[napi(js_name = "treeRootChildCount")]
pub fn tree_root_child_count(tree: &External<tree_sitter::Tree>) -> u32 {
    tree.root_node().named_child_count() as u32
}

/// Check whether any node in the tree has the given type name.
#[napi(js_name = "treeContainsNodeType")]
pub fn tree_contains_node_type(tree: &External<tree_sitter::Tree>, node_type: String) -> bool {
    ts_pack_core::tree_contains_node_type(tree, &node_type)
}

/// Check whether the tree contains any ERROR or MISSING nodes.
#[napi(js_name = "treeHasErrorNodes")]
pub fn tree_has_error_nodes(tree: &External<tree_sitter::Tree>) -> bool {
    ts_pack_core::tree_has_error_nodes(tree)
}

// ---------------------------------------------------------------------------
// Process with config
// ---------------------------------------------------------------------------

/// Configuration for the `process` function.
#[napi(object)]
pub struct JsProcessConfig {
    pub language: String,
    pub structure: Option<bool>,
    pub imports: Option<bool>,
    pub exports: Option<bool>,
    pub comments: Option<bool>,
    pub docstrings: Option<bool>,
    pub symbols: Option<bool>,
    pub diagnostics: Option<bool>,
    pub chunk_max_size: Option<u32>,
}

impl From<JsProcessConfig> for ts_pack_core::ProcessConfig {
    fn from(js: JsProcessConfig) -> Self {
        Self {
            language: js.language,
            structure: js.structure.unwrap_or(true),
            imports: js.imports.unwrap_or(true),
            exports: js.exports.unwrap_or(true),
            comments: js.comments.unwrap_or(true),
            docstrings: js.docstrings.unwrap_or(true),
            symbols: js.symbols.unwrap_or(true),
            diagnostics: js.diagnostics.unwrap_or(true),
            chunk_max_size: js.chunk_max_size.map(|v| v as usize),
        }
    }
}

/// Process source code using a config and return a JavaScript object with metadata and chunks.
#[napi(js_name = "process")]
pub fn process(source: String, config: JsProcessConfig) -> napi::Result<serde_json::Value> {
    let core_config: ts_pack_core::ProcessConfig = config.into();
    let result = ts_pack_core::process(&source, &core_config).map_err(|e| napi::Error::from_reason(format!("{e}")))?;
    serde_json::to_value(&result).map_err(|e| napi::Error::from_reason(format!("serialization failed: {e}")))
}
