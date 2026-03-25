use std::sync::Mutex;
use wasm_bindgen::prelude::*;

// Provide wide-character C functions that tree-sitter external scanners import
// from the "env" namespace. These are simple ASCII-range implementations
// sufficient for parser operation in WASM.
#[unsafe(no_mangle)]
pub extern "C" fn iswspace(c: u32) -> i32 {
    matches!(c, 0x09..=0x0D | 0x20 | 0x85 | 0xA0 | 0x1680 | 0x2000..=0x200A | 0x2028 | 0x2029 | 0x202F | 0x205F | 0x3000)
        as i32
}

#[unsafe(no_mangle)]
pub extern "C" fn iswalnum(c: u32) -> i32 {
    char::from_u32(c).is_some_and(|ch| ch.is_alphanumeric()) as i32
}

#[unsafe(no_mangle)]
pub extern "C" fn towupper(c: u32) -> u32 {
    char::from_u32(c)
        .and_then(|ch| ch.to_uppercase().next())
        .map_or(c, |ch| ch as u32)
}

#[unsafe(no_mangle)]
pub extern "C" fn iswalpha(c: u32) -> i32 {
    char::from_u32(c).is_some_and(|ch| ch.is_alphabetic()) as i32
}

/// Returns an array of all available language names.
#[wasm_bindgen(js_name = "availableLanguages")]
pub fn available_languages() -> Vec<JsValue> {
    tree_sitter_language_pack::available_languages()
        .into_iter()
        .map(JsValue::from)
        .collect()
}

/// Checks whether a language with the given name is available.
#[wasm_bindgen(js_name = "hasLanguage")]
pub fn has_language(name: &str) -> bool {
    tree_sitter_language_pack::has_language(name)
}

/// Detect language name from a file path or extension.
/// Returns null if the extension is not recognized.
#[wasm_bindgen(js_name = "detectLanguage")]
pub fn detect_language(path: &str) -> Option<String> {
    tree_sitter_language_pack::detect_language_from_path(path).map(String::from)
}

/// Returns the number of available languages.
#[wasm_bindgen(js_name = "languageCount")]
pub fn language_count() -> u32 {
    tree_sitter_language_pack::language_count() as u32
}

/// Detect language name from file content (shebang-based detection).
/// Returns null if the content does not contain a recognized shebang.
#[wasm_bindgen(js_name = "detectLanguageFromContent")]
pub fn detect_language_from_content(content: &str) -> Option<String> {
    tree_sitter_language_pack::detect_language_from_content(content).map(String::from)
}

/// Returns extension ambiguity information for the given file extension as a JSON string.
/// Returns null if the extension is not ambiguous.
/// When non-null, parses to an object with "assigned" (string) and "alternatives" (string[]) fields.
#[wasm_bindgen(js_name = "extensionAmbiguity")]
pub fn extension_ambiguity(ext: &str) -> Option<String> {
    tree_sitter_language_pack::extension_ambiguity_json(ext)
}

/// Returns the bundled highlights query for the given language, or null.
#[wasm_bindgen(js_name = "getHighlightsQuery")]
pub fn get_highlights_query(language: &str) -> Option<String> {
    tree_sitter_language_pack::get_highlights_query(language).map(String::from)
}

/// Returns the bundled injections query for the given language, or null.
#[wasm_bindgen(js_name = "getInjectionsQuery")]
pub fn get_injections_query(language: &str) -> Option<String> {
    tree_sitter_language_pack::get_injections_query(language).map(String::from)
}

/// Returns the bundled locals query for the given language, or null.
#[wasm_bindgen(js_name = "getLocalsQuery")]
pub fn get_locals_query(language: &str) -> Option<String> {
    tree_sitter_language_pack::get_locals_query(language).map(String::from)
}

/// Returns the raw TSLanguage pointer as a u32 for wasm32 interop.
///
/// Throws an error if the language is not found.
#[wasm_bindgen(js_name = "getLanguagePtr")]
pub fn get_language_ptr(name: &str) -> Result<u32, JsValue> {
    let language = tree_sitter_language_pack::get_language(name).map_err(|e| JsValue::from_str(&format!("{e}")))?;
    let ptr = language.into_raw() as u32;
    Ok(ptr)
}

// ---------------------------------------------------------------------------
// Tree wrapper for opaque handle
// ---------------------------------------------------------------------------

#[wasm_bindgen]
pub struct WasmTree {
    inner: Mutex<tree_sitter::Tree>,
}

/// Parse a source string using the named language and return an opaque tree handle.
///
/// Throws an error if the language is not found or parsing fails.
#[wasm_bindgen(js_name = "parseString")]
pub fn parse_string(language: &str, source: &str) -> Result<WasmTree, JsValue> {
    let tree = tree_sitter_language_pack::parse_string(language, source.as_bytes())
        .map_err(|e| JsValue::from_str(&format!("{e}")))?;
    Ok(WasmTree {
        inner: Mutex::new(tree),
    })
}

/// Get the type name of the root node.
#[wasm_bindgen(js_name = "treeRootNodeType")]
pub fn tree_root_node_type(tree: &WasmTree) -> Result<String, JsValue> {
    let guard = tree
        .inner
        .lock()
        .map_err(|e| JsValue::from_str(&format!("lock error: {e}")))?;
    Ok(guard.root_node().kind().to_string())
}

/// Get the number of named children of the root node.
#[wasm_bindgen(js_name = "treeRootChildCount")]
pub fn tree_root_child_count(tree: &WasmTree) -> Result<u32, JsValue> {
    let guard = tree
        .inner
        .lock()
        .map_err(|e| JsValue::from_str(&format!("lock error: {e}")))?;
    Ok(guard.root_node().named_child_count() as u32)
}

/// Check whether any node in the tree has the given type name.
#[wasm_bindgen(js_name = "treeContainsNodeType")]
pub fn tree_contains_node_type(tree: &WasmTree, node_type: &str) -> Result<bool, JsValue> {
    let guard = tree
        .inner
        .lock()
        .map_err(|e| JsValue::from_str(&format!("lock error: {e}")))?;
    Ok(tree_sitter_language_pack::tree_contains_node_type(&guard, node_type))
}

/// Check whether the tree contains any ERROR or MISSING nodes.
#[wasm_bindgen(js_name = "treeHasErrorNodes")]
pub fn tree_has_error_nodes(tree: &WasmTree) -> Result<bool, JsValue> {
    let guard = tree
        .inner
        .lock()
        .map_err(|e| JsValue::from_str(&format!("lock error: {e}")))?;
    Ok(tree_sitter_language_pack::tree_has_error_nodes(&guard))
}

/// Free the tree handle (called automatically by JS GC, but can be called manually).
#[wasm_bindgen(js_name = "freeTree")]
pub fn free_tree(_tree: WasmTree) {
    // Dropping the WasmTree frees the underlying tree_sitter::Tree
}

// ---------------------------------------------------------------------------
// Process: unified API
// ---------------------------------------------------------------------------

/// Process source code and extract metadata + chunks as a JavaScript object.
///
/// `config` is a JS object with fields:
/// - `language` (string, required): the language name
/// - `chunk_max_size` (number, optional): maximum chunk size in bytes (default: 1500)
#[wasm_bindgen(js_name = "process")]
pub fn process(source: &str, config: JsValue) -> Result<JsValue, JsValue> {
    let config_json: serde_json::Value = js_sys::JSON::stringify(&config)
        .map_err(|e| JsValue::from_str(&format!("failed to stringify config: {e:?}")))?
        .as_string()
        .ok_or_else(|| JsValue::from_str("config stringify returned non-string"))
        .and_then(|s| {
            serde_json::from_str(&s).map_err(|e| JsValue::from_str(&format!("failed to parse config JSON: {e}")))
        })?;

    let core_config: tree_sitter_language_pack::ProcessConfig =
        serde_json::from_value(config_json).map_err(|e| JsValue::from_str(&format!("invalid config: {e}")))?;

    let result =
        tree_sitter_language_pack::process(source, &core_config).map_err(|e| JsValue::from_str(&format!("{e}")))?;
    let json_str =
        serde_json::to_string(&result).map_err(|e| JsValue::from_str(&format!("serialization failed: {e}")))?;
    js_sys::JSON::parse(&json_str)
}

// ---------------------------------------------------------------------------
// Download/configure API (not supported in WASM)
// ---------------------------------------------------------------------------

/// Configure the language pack (stub for WASM).
/// WASM cannot perform network I/O, so download functions are not supported.
#[wasm_bindgen(js_name = "configure")]
pub fn configure(_config: &str) -> Result<(), JsValue> {
    Err(JsValue::from_str(
        "configure not supported in WASM - no persistent cache",
    ))
}

/// Initialize the language pack (stub for WASM).
/// WASM cannot perform network I/O, so download functions are not supported.
#[wasm_bindgen(js_name = "init")]
pub fn init(_config: &str) -> Result<(), JsValue> {
    Err(JsValue::from_str(
        "init/download not supported in WASM - no network I/O",
    ))
}

/// Download specific languages (not supported in WASM).
#[wasm_bindgen(js_name = "download")]
pub fn download(_languages: js_sys::Array) -> Result<i32, JsValue> {
    Err(JsValue::from_str("download not supported in WASM - no network I/O"))
}

/// Download all languages (not supported in WASM).
#[wasm_bindgen(js_name = "downloadAll")]
pub fn download_all() -> Result<i32, JsValue> {
    Err(JsValue::from_str("downloadAll not supported in WASM - no network I/O"))
}

/// Get manifest languages (not supported in WASM).
#[wasm_bindgen(js_name = "manifestLanguages")]
pub fn manifest_languages() -> Result<js_sys::Array, JsValue> {
    Err(JsValue::from_str(
        "manifestLanguages not supported in WASM - no network I/O",
    ))
}

/// Get downloaded languages (stub for WASM).
/// Returns empty array since WASM has no persistent cache.
#[wasm_bindgen(js_name = "downloadedLanguages")]
pub fn downloaded_languages() -> Result<js_sys::Array, JsValue> {
    Ok(js_sys::Array::new())
}

/// Clean cache (stub for WASM).
/// WASM has no persistent cache, so this is a no-op.
#[wasm_bindgen(js_name = "cleanCache")]
pub fn clean_cache() -> Result<(), JsValue> {
    Ok(())
}

/// Get cache directory (not supported in WASM).
#[wasm_bindgen(js_name = "cacheDir")]
pub fn cache_dir() -> Result<String, JsValue> {
    Err(JsValue::from_str(
        "cacheDir not supported in WASM - no persistent cache",
    ))
}
