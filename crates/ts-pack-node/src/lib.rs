use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Returns an array of all available language names.
#[napi(js_name = "availableLanguages")]
pub fn available_languages() -> Vec<String> {
    tree_sitter_language_pack::available_languages()
}

/// Checks whether a language with the given name is available.
#[napi(js_name = "hasLanguage")]
pub fn has_language(name: String) -> bool {
    tree_sitter_language_pack::has_language(&name)
}

/// Detect language name from a file path or extension.
/// Returns null if the extension is not recognized.
#[napi(js_name = "detectLanguage")]
pub fn detect_language(path: String) -> Option<String> {
    tree_sitter_language_pack::detect_language_from_path(&path).map(String::from)
}

/// Returns the number of available languages.
#[napi(js_name = "languageCount")]
pub fn language_count() -> u32 {
    tree_sitter_language_pack::language_count() as u32
}

/// Detect language name from file content (shebang-based detection).
/// Returns null if the content does not contain a recognized shebang.
#[napi(js_name = "detectLanguageFromContent")]
pub fn detect_language_from_content(content: String) -> Option<String> {
    tree_sitter_language_pack::detect_language_from_content(&content).map(String::from)
}

/// Returns extension ambiguity information for the given file extension.
/// Returns null if the extension is not ambiguous.
/// When non-null, returns an object with "assigned" (string) and "alternatives" (string[]) fields.
#[napi(js_name = "extensionAmbiguity")]
pub fn extension_ambiguity(ext: String) -> Option<serde_json::Value> {
    tree_sitter_language_pack::extension_ambiguity(&ext).map(|(assigned, alts)| {
        serde_json::json!({
            "assigned": assigned,
            "alternatives": alts,
        })
    })
}

/// Returns the bundled highlights query for the given language, or null.
#[napi(js_name = "getHighlightsQuery")]
pub fn get_highlights_query(language: String) -> Option<String> {
    tree_sitter_language_pack::get_highlights_query(&language).map(String::from)
}

/// Returns the bundled injections query for the given language, or null.
#[napi(js_name = "getInjectionsQuery")]
pub fn get_injections_query(language: String) -> Option<String> {
    tree_sitter_language_pack::get_injections_query(&language).map(String::from)
}

/// Returns the bundled locals query for the given language, or null.
#[napi(js_name = "getLocalsQuery")]
pub fn get_locals_query(language: String) -> Option<String> {
    tree_sitter_language_pack::get_locals_query(&language).map(String::from)
}

/// Returns the raw TSLanguage pointer for interop with node-tree-sitter.
///
/// Throws an error if the language is not found.
#[napi(js_name = "getLanguagePtr")]
pub fn get_language_ptr(name: String) -> napi::Result<i64> {
    let language =
        tree_sitter_language_pack::get_language(&name).map_err(|e| napi::Error::from_reason(format!("{e}")))?;
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
    let tree = tree_sitter_language_pack::parse_string(&language, source.as_bytes())
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
    tree_sitter_language_pack::tree_contains_node_type(tree, &node_type)
}

/// Check whether the tree contains any ERROR or MISSING nodes.
#[napi(js_name = "treeHasErrorNodes")]
pub fn tree_has_error_nodes(tree: &External<tree_sitter::Tree>) -> bool {
    tree_sitter_language_pack::tree_has_error_nodes(tree)
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

impl From<JsProcessConfig> for tree_sitter_language_pack::ProcessConfig {
    fn from(js: JsProcessConfig) -> Self {
        Self {
            language: std::borrow::Cow::Owned(js.language),
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
///
/// Accepts both camelCase and snake_case config keys (auto-normalized to snake_case).
/// Returns camelCase keys in the result for JavaScript convention.
#[napi(js_name = "process")]
pub fn process(source: String, config: serde_json::Value) -> napi::Result<serde_json::Value> {
    // Normalize config keys to snake_case (accepts both camelCase and snake_case input)
    let normalized = tree_sitter_language_pack::json_utils::camel_to_snake(config);
    let core_config: tree_sitter_language_pack::ProcessConfig =
        serde_json::from_value(normalized).map_err(|e| napi::Error::from_reason(format!("invalid config: {e}")))?;
    let result = tree_sitter_language_pack::process(&source, &core_config)
        .map_err(|e| napi::Error::from_reason(format!("{e}")))?;
    let json =
        serde_json::to_value(&result).map_err(|e| napi::Error::from_reason(format!("serialization failed: {e}")))?;
    // Convert result keys to camelCase for JS convention
    Ok(tree_sitter_language_pack::json_utils::snake_to_camel(json))
}

// ---------------------------------------------------------------------------
// Download and configure API
// ---------------------------------------------------------------------------

/// Configuration for download and cache management.
#[napi(object)]
#[derive(Default)]
pub struct JsPackConfig {
    pub cache_dir: Option<String>,
    pub languages: Option<Vec<String>>,
    pub groups: Option<Vec<String>>,
}

impl From<JsPackConfig> for tree_sitter_language_pack::PackConfig {
    fn from(js: JsPackConfig) -> Self {
        Self {
            cache_dir: js.cache_dir.map(std::path::PathBuf::from),
            languages: js.languages,
            groups: js.groups,
        }
    }
}

/// Initialize download system with configuration and pre-download all specified languages.
///
/// Throws an error if configuration or download fails.
#[napi(js_name = "init")]
pub fn js_init(config: Option<JsPackConfig>) -> napi::Result<()> {
    let pack_config = tree_sitter_language_pack::PackConfig::from(config.unwrap_or_default());
    tree_sitter_language_pack::init(&pack_config).map_err(|e| napi::Error::from_reason(format!("{e}")))
}

/// Configure the cache directory without downloading.
///
/// Throws an error if configuration fails.
#[napi(js_name = "configure")]
pub fn js_configure(config: JsPackConfig) -> napi::Result<()> {
    let pack_config = tree_sitter_language_pack::PackConfig::from(config);
    tree_sitter_language_pack::configure(&pack_config).map_err(|e| napi::Error::from_reason(format!("{e}")))
}

/// Download specific languages by name.
///
/// Returns the number of languages successfully downloaded.
/// Throws an error if download fails.
#[napi(js_name = "download")]
pub fn js_download(names: Vec<String>) -> napi::Result<u32> {
    let name_refs: Vec<&str> = names.iter().map(|s| s.as_str()).collect();
    tree_sitter_language_pack::download(&name_refs)
        .map(|count| count as u32)
        .map_err(|e| napi::Error::from_reason(format!("{e}")))
}

/// Download all 170+ available languages from the remote manifest.
///
/// Returns the number of languages successfully downloaded.
/// Throws an error if download fails.
#[napi(js_name = "downloadAll")]
pub fn js_download_all() -> napi::Result<u32> {
    tree_sitter_language_pack::download_all()
        .map(|count| count as u32)
        .map_err(|e| napi::Error::from_reason(format!("{e}")))
}

/// Get all available languages from the remote manifest.
///
/// Returns an array of language names. Throws an error if manifest fetch fails.
#[napi(js_name = "manifestLanguages")]
pub fn js_manifest_languages() -> napi::Result<Vec<String>> {
    tree_sitter_language_pack::manifest_languages().map_err(|e| napi::Error::from_reason(format!("{e}")))
}

/// Get all languages that have been downloaded and cached locally.
///
/// Returns an array of language names currently in the cache.
#[napi(js_name = "downloadedLanguages")]
pub fn js_downloaded_languages() -> Vec<String> {
    tree_sitter_language_pack::downloaded_languages()
}

/// Delete all cached parser files.
///
/// Throws an error if cache deletion fails.
#[napi(js_name = "cleanCache")]
pub fn js_clean_cache() -> napi::Result<()> {
    tree_sitter_language_pack::clean_cache().map_err(|e| napi::Error::from_reason(format!("{e}")))
}

/// Get the effective cache directory being used.
///
/// Returns the path as a string. Throws an error if cache directory cannot be determined.
#[napi(js_name = "cacheDir")]
pub fn js_cache_dir() -> napi::Result<String> {
    tree_sitter_language_pack::cache_dir()
        .map(|path| path.to_string_lossy().to_string())
        .map_err(|e| napi::Error::from_reason(format!("{e}")))
}
