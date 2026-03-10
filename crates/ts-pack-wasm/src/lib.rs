use std::sync::Mutex;
use wasm_bindgen::prelude::*;

// Provide wide-character C functions that tree-sitter external scanners import
// from the "env" namespace. These are simple ASCII-range implementations
// sufficient for parser operation in WASM.
#[unsafe(no_mangle)]
pub extern "C" fn iswspace(c: u32) -> i32 {
    matches!(c, 0x09..=0x0D | 0x20 | 0x85 | 0xA0 | 0x1680 | 0x2000..=0x200A | 0x2028 | 0x2029 | 0x202F | 0x205F | 0x3000) as i32
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
    ts_pack_core::available_languages()
        .into_iter()
        .map(JsValue::from)
        .collect()
}

/// Checks whether a language with the given name is available.
#[wasm_bindgen(js_name = "hasLanguage")]
pub fn has_language(name: &str) -> bool {
    ts_pack_core::has_language(name)
}

/// Returns the number of available languages.
#[wasm_bindgen(js_name = "languageCount")]
pub fn language_count() -> u32 {
    ts_pack_core::language_count() as u32
}

/// Returns the raw TSLanguage pointer as a u32 for wasm32 interop.
///
/// Throws an error if the language is not found.
#[wasm_bindgen(js_name = "getLanguagePtr")]
pub fn get_language_ptr(name: &str) -> Result<u32, JsValue> {
    let language = ts_pack_core::get_language(name)
        .map_err(|e| JsValue::from_str(&format!("{e}")))?;
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
    let tree = ts_pack_core::parse_string(language, source.as_bytes())
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
    Ok(ts_pack_core::tree_contains_node_type(&guard, node_type))
}

/// Check whether the tree contains any ERROR or MISSING nodes.
#[wasm_bindgen(js_name = "treeHasErrorNodes")]
pub fn tree_has_error_nodes(tree: &WasmTree) -> Result<bool, JsValue> {
    let guard = tree
        .inner
        .lock()
        .map_err(|e| JsValue::from_str(&format!("lock error: {e}")))?;
    Ok(ts_pack_core::tree_has_error_nodes(&guard))
}

/// Free the tree handle (called automatically by JS GC, but can be called manually).
#[wasm_bindgen(js_name = "freeTree")]
pub fn free_tree(_tree: WasmTree) {
    // Dropping the WasmTree frees the underlying tree_sitter::Tree
}

// ---------------------------------------------------------------------------
// Intel: process / processAndChunk
// ---------------------------------------------------------------------------

/// Process source code and extract file intelligence as a JavaScript object.
#[wasm_bindgen(js_name = "process")]
pub fn process(source: &str, language: &str) -> Result<JsValue, JsValue> {
    let intel = ts_pack_core::process(source, language)
        .map_err(|e| JsValue::from_str(&format!("{e}")))?;
    let json_str = serde_json::to_string(&intel)
        .map_err(|e| JsValue::from_str(&format!("serialization failed: {e}")))?;
    js_sys::JSON::parse(&json_str)
}

/// Process and chunk source code, returning a JavaScript object with intelligence and chunks.
#[wasm_bindgen(js_name = "processAndChunk")]
pub fn process_and_chunk(source: &str, language: &str, max_chunk_size: usize) -> Result<JsValue, JsValue> {
    let (intel, chunks) = ts_pack_core::process_and_chunk(source, language, max_chunk_size)
        .map_err(|e| JsValue::from_str(&format!("{e}")))?;
    let result = serde_json::json!({
        "intelligence": intel,
        "chunks": chunks,
    });
    let json_str = serde_json::to_string(&result)
        .map_err(|e| JsValue::from_str(&format!("serialization failed: {e}")))?;
    js_sys::JSON::parse(&json_str)
}
