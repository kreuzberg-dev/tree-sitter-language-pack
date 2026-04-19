//! C-FFI bindings for tree-sitter-language-pack.
//!
//! This crate wraps `ts-pack-core` and exposes a C-compatible API for creating
//! a language registry, querying available languages, and obtaining raw
//! `TSLanguage` pointers suitable for use from C or any language with C-FFI support.

use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::panic;
use std::ptr;

use tree_sitter::ffi::TSLanguage;
use tree_sitter_language_pack::LanguageRegistry;

#[cfg(feature = "download")]
use tree_sitter_language_pack::PackConfig;

// ---------------------------------------------------------------------------
// Opaque handles
// ---------------------------------------------------------------------------

/// Opaque handle to a language registry.
/// Created with `ts_pack_registry_new` and freed with `ts_pack_registry_free`.
pub struct TsPackRegistry {
    inner: LanguageRegistry,
    /// Cached sorted list of language names kept in sync with the registry.
    cached_names: Vec<CString>,
}

/// Opaque handle to a parsed syntax tree.
/// Created with `ts_pack_parse_string` and freed with `ts_pack_tree_free`.
pub struct TsPackTree {
    inner: tree_sitter::Tree,
}

// ---------------------------------------------------------------------------
// Thread-local error
// ---------------------------------------------------------------------------

thread_local! {
    static LAST_ERROR: RefCell<Option<CString>> = const { RefCell::new(None) };
}

fn set_last_error(msg: &str) {
    let c = CString::new(msg.replace('\0', "")).unwrap_or_default();
    LAST_ERROR.with(|e| {
        *e.borrow_mut() = Some(c);
    });
}

fn clear_last_error() {
    LAST_ERROR.with(|e| {
        *e.borrow_mut() = None;
    });
}

// ---------------------------------------------------------------------------
// Panic shield macro
// ---------------------------------------------------------------------------

/// Runs a closure inside `catch_unwind`. On panic the error is stored in the
/// thread-local `LAST_ERROR` and `$default` is returned.
macro_rules! ffi_guard {
    ($default:expr, $body:expr) => {{
        match panic::catch_unwind(panic::AssertUnwindSafe(|| $body)) {
            Ok(val) => val,
            Err(e) => {
                let msg = if let Some(s) = e.downcast_ref::<&str>() {
                    s.to_string()
                } else if let Some(s) = e.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "unknown panic".to_string()
                };
                set_last_error(&format!("panic: {msg}"));
                $default
            }
        }
    }};
}

// ---------------------------------------------------------------------------
// FFI functions
// ---------------------------------------------------------------------------

/// Create a new language registry.
///
/// Returns a pointer to the registry, or null on failure.
/// The caller must free the registry with `ts_pack_registry_free`.
///
/// # Safety
///
/// The returned pointer must be freed with `ts_pack_registry_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_registry_new() -> *mut TsPackRegistry {
    ffi_guard!(ptr::null_mut(), {
        clear_last_error();
        let inner = LanguageRegistry::new();
        #[cfg(feature = "download")]
        if let Ok(cache_dir) = tree_sitter_language_pack::cache_dir() {
            inner.add_extra_libs_dir(cache_dir);
        }
        let names: Vec<CString> = inner
            .available_languages()
            .into_iter()
            .filter_map(|n| CString::new(n).ok())
            .collect();
        let registry = Box::new(TsPackRegistry {
            inner,
            cached_names: names,
        });
        Box::into_raw(registry)
    })
}

/// Free a registry previously created with `ts_pack_registry_new`.
///
/// Passing a null pointer is a safe no-op.
///
/// # Safety
///
/// `registry` must be a pointer returned by `ts_pack_registry_new`, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_registry_free(registry: *mut TsPackRegistry) {
    ffi_guard!((), {
        if !registry.is_null() {
            // SAFETY: pointer was created by Box::into_raw in ts_pack_registry_new
            unsafe {
                drop(Box::from_raw(registry));
            }
        }
    });
}

/// Get a raw `TSLanguage` pointer for the given language name.
///
/// Returns null on error (check `ts_pack_last_error` for details).
/// The returned pointer is valid for the lifetime of the registry.
///
/// # Safety
///
/// `registry` must be a valid pointer returned by `ts_pack_registry_new`, or null.
/// `name` must be a valid null-terminated UTF-8 C string, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_get_language(
    registry: *const TsPackRegistry,
    name: *const c_char,
) -> *const TSLanguage {
    ffi_guard!(ptr::null(), {
        clear_last_error();
        if registry.is_null() {
            set_last_error("registry pointer is null");
            return ptr::null();
        }
        if name.is_null() {
            set_last_error("name pointer is null");
            return ptr::null();
        }
        // SAFETY: caller guarantees valid pointer from ts_pack_registry_new
        let reg = unsafe { &*registry };
        // SAFETY: caller guarantees valid null-terminated string; null check above
        let name_str = unsafe { CStr::from_ptr(name) };
        let name_str = match name_str.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_last_error(&format!("invalid UTF-8 in name: {e}"));
                return ptr::null();
            }
        };
        match reg.inner.get_language(name_str) {
            Ok(lang) => lang.into_raw(),
            Err(e) => {
                set_last_error(&e.to_string());
                ptr::null()
            }
        }
    })
}

/// Return the number of available languages.
///
/// Returns 0 if the registry pointer is null.
///
/// # Safety
///
/// `registry` must be a valid pointer returned by `ts_pack_registry_new`, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_language_count(registry: *const TsPackRegistry) -> usize {
    ffi_guard!(0, {
        clear_last_error();
        if registry.is_null() {
            set_last_error("registry pointer is null");
            return 0;
        }
        // SAFETY: caller guarantees valid pointer; null check above
        let reg = unsafe { &*registry };
        reg.cached_names.len()
    })
}

/// Get the language name at the given index.
///
/// Returns a newly-allocated C string that the caller must free with
/// `ts_pack_free_string`. Returns null if the index is out of bounds or
/// the registry pointer is null.
///
/// # Safety
///
/// `registry` must be a valid pointer returned by `ts_pack_registry_new`, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_language_name_at(registry: *const TsPackRegistry, index: usize) -> *const c_char {
    ffi_guard!(ptr::null(), {
        clear_last_error();
        if registry.is_null() {
            set_last_error("registry pointer is null");
            return ptr::null();
        }
        // SAFETY: caller guarantees valid pointer; null check above
        let reg = unsafe { &*registry };
        match reg.cached_names.get(index) {
            Some(name) => {
                // Clone so the caller owns the memory and can free it independently
                let cloned = name.clone();
                CString::into_raw(cloned) as *const c_char
            }
            None => {
                set_last_error(&format!(
                    "index {index} out of bounds (count: {})",
                    reg.cached_names.len()
                ));
                ptr::null()
            }
        }
    })
}

/// Check whether the registry contains a language with the given name.
///
/// Returns false if the registry or name pointer is null.
///
/// # Safety
///
/// `registry` must be a valid pointer returned by `ts_pack_registry_new`, or null.
/// `name` must be a valid null-terminated UTF-8 C string, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_has_language(registry: *const TsPackRegistry, name: *const c_char) -> bool {
    ffi_guard!(false, {
        clear_last_error();
        if registry.is_null() {
            set_last_error("registry pointer is null");
            return false;
        }
        if name.is_null() {
            set_last_error("name pointer is null");
            return false;
        }
        // SAFETY: caller guarantees valid pointer; null check above
        let reg = unsafe { &*registry };
        // SAFETY: caller guarantees valid null-terminated string; null check above
        let name_str = unsafe { CStr::from_ptr(name) };
        match name_str.to_str() {
            Ok(s) => reg.inner.has_language(s),
            Err(e) => {
                set_last_error(&format!("invalid UTF-8 in name: {e}"));
                false
            }
        }
    })
}

/// Detect language name from a file path.
///
/// Returns a newly allocated null-terminated UTF-8 string with the language name,
/// or null if the extension is not recognized. The caller must free the returned
/// pointer with `ts_pack_free_string`.
///
/// # Safety
///
/// `path` must be a valid null-terminated UTF-8 C string, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_detect_language(path: *const c_char) -> *mut c_char {
    ffi_guard!(ptr::null_mut(), {
        clear_last_error();
        if path.is_null() {
            set_last_error("path pointer is null");
            return ptr::null_mut();
        }
        // SAFETY: caller guarantees valid null-terminated string; null check above
        let path_str = unsafe { CStr::from_ptr(path) };
        match path_str.to_str() {
            Ok(s) => match tree_sitter_language_pack::detect_language_from_path(s) {
                Some(lang) => CString::new(lang).map(CString::into_raw).unwrap_or(ptr::null_mut()),
                None => ptr::null_mut(),
            },
            Err(e) => {
                set_last_error(&format!("invalid UTF-8 in path: {e}"));
                ptr::null_mut()
            }
        }
    })
}

/// Detect language name from file content using shebang-based detection.
///
/// Returns a newly allocated null-terminated UTF-8 string with the language name,
/// or null if no shebang is recognized. The caller must free the returned
/// pointer with `ts_pack_free_string`.
///
/// # Safety
///
/// `content` must be a valid null-terminated UTF-8 C string, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_detect_language_from_content(content: *const c_char) -> *mut c_char {
    ffi_guard!(ptr::null_mut(), {
        clear_last_error();
        if content.is_null() {
            set_last_error("content pointer is null");
            return ptr::null_mut();
        }
        // SAFETY: caller guarantees valid null-terminated string; null check above
        let content_str = unsafe { CStr::from_ptr(content) };
        match content_str.to_str() {
            Ok(s) => match tree_sitter_language_pack::detect_language_from_content(s) {
                Some(lang) => CString::new(lang).map(CString::into_raw).unwrap_or(ptr::null_mut()),
                None => ptr::null_mut(),
            },
            Err(e) => {
                set_last_error(&format!("invalid UTF-8 in content: {e}"));
                ptr::null_mut()
            }
        }
    })
}

/// Detect language name from a bare file extension (without leading dot).
///
/// Returns a newly allocated null-terminated UTF-8 string with the language name,
/// or null if the extension is not recognized. The caller must free the returned
/// pointer with `ts_pack_free_string`.
///
/// # Safety
///
/// `ext` must be a valid null-terminated UTF-8 C string, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_detect_language_from_extension(ext: *const c_char) -> *mut c_char {
    ffi_guard!(ptr::null_mut(), {
        clear_last_error();
        if ext.is_null() {
            set_last_error("ext pointer is null");
            return ptr::null_mut();
        }
        // SAFETY: caller guarantees valid null-terminated string; null check above
        let ext_str = unsafe { CStr::from_ptr(ext) };
        match ext_str.to_str() {
            Ok(s) => match tree_sitter_language_pack::detect_language_from_extension(s) {
                Some(lang) => CString::new(lang).map(CString::into_raw).unwrap_or(ptr::null_mut()),
                None => ptr::null_mut(),
            },
            Err(e) => {
                set_last_error(&format!("invalid UTF-8 in ext: {e}"));
                ptr::null_mut()
            }
        }
    })
}

/// Detect language name from a file path based on its extension.
///
/// This is an explicit alias of `ts_pack_detect_language` for API consistency.
///
/// Returns a newly allocated null-terminated UTF-8 string with the language name,
/// or null if the extension is not recognized. The caller must free the returned
/// pointer with `ts_pack_free_string`.
///
/// # Safety
///
/// `path` must be a valid null-terminated UTF-8 C string, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_detect_language_from_path(path: *const c_char) -> *mut c_char {
    ffi_guard!(ptr::null_mut(), {
        clear_last_error();
        if path.is_null() {
            set_last_error("path pointer is null");
            return ptr::null_mut();
        }
        // SAFETY: caller guarantees valid null-terminated string; null check above
        let path_str = unsafe { CStr::from_ptr(path) };
        match path_str.to_str() {
            Ok(s) => match tree_sitter_language_pack::detect_language_from_path(s) {
                Some(lang) => CString::new(lang).map(CString::into_raw).unwrap_or(ptr::null_mut()),
                None => ptr::null_mut(),
            },
            Err(e) => {
                set_last_error(&format!("invalid UTF-8 in path: {e}"));
                ptr::null_mut()
            }
        }
    })
}

/// Returns extension ambiguity information for the given file extension as a JSON C string.
///
/// Returns null if the extension is not ambiguous.
/// When non-null, the JSON object has "assigned" (string) and "alternatives" (string[]) fields.
/// The caller must free the returned pointer with `ts_pack_free_string`.
///
/// # Safety
///
/// `ext` must be a valid null-terminated UTF-8 C string, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_extension_ambiguity(ext: *const c_char) -> *mut c_char {
    ffi_guard!(ptr::null_mut(), {
        clear_last_error();
        if ext.is_null() {
            set_last_error("ext pointer is null");
            return ptr::null_mut();
        }
        // SAFETY: caller guarantees valid null-terminated string; null check above
        let ext_str = unsafe { CStr::from_ptr(ext) };
        match ext_str.to_str() {
            Ok(s) => match tree_sitter_language_pack::extension_ambiguity(s) {
                Some((assigned, alts)) => {
                    let val = serde_json::json!({
                        "assigned": assigned,
                        "alternatives": alts,
                    });
                    match serde_json::to_string(&val) {
                        Ok(json) => CString::new(json).map(CString::into_raw).unwrap_or(ptr::null_mut()),
                        Err(e) => {
                            set_last_error(&format!("serialization failed: {e}"));
                            ptr::null_mut()
                        }
                    }
                }
                None => ptr::null_mut(),
            },
            Err(e) => {
                set_last_error(&format!("invalid UTF-8 in ext: {e}"));
                ptr::null_mut()
            }
        }
    })
}

/// Returns the bundled highlights query for the given language as a C string.
///
/// Returns null if no bundled query is available for the language.
/// The caller must free the returned pointer with `ts_pack_free_string`.
///
/// # Safety
///
/// `language` must be a valid null-terminated UTF-8 C string, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_get_highlights_query(language: *const c_char) -> *mut c_char {
    ffi_guard!(ptr::null_mut(), {
        clear_last_error();
        if language.is_null() {
            set_last_error("language pointer is null");
            return ptr::null_mut();
        }
        // SAFETY: caller guarantees valid null-terminated string; null check above
        let lang_str = unsafe { CStr::from_ptr(language) };
        match lang_str.to_str() {
            Ok(s) => match tree_sitter_language_pack::get_highlights_query(s) {
                Some(q) => CString::new(q).map(CString::into_raw).unwrap_or(ptr::null_mut()),
                None => ptr::null_mut(),
            },
            Err(e) => {
                set_last_error(&format!("invalid UTF-8 in language: {e}"));
                ptr::null_mut()
            }
        }
    })
}

/// Returns the bundled injections query for the given language as a C string.
///
/// Returns null if no bundled query is available for the language.
/// The caller must free the returned pointer with `ts_pack_free_string`.
///
/// # Safety
///
/// `language` must be a valid null-terminated UTF-8 C string, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_get_injections_query(language: *const c_char) -> *mut c_char {
    ffi_guard!(ptr::null_mut(), {
        clear_last_error();
        if language.is_null() {
            set_last_error("language pointer is null");
            return ptr::null_mut();
        }
        // SAFETY: caller guarantees valid null-terminated string; null check above
        let lang_str = unsafe { CStr::from_ptr(language) };
        match lang_str.to_str() {
            Ok(s) => match tree_sitter_language_pack::get_injections_query(s) {
                Some(q) => CString::new(q).map(CString::into_raw).unwrap_or(ptr::null_mut()),
                None => ptr::null_mut(),
            },
            Err(e) => {
                set_last_error(&format!("invalid UTF-8 in language: {e}"));
                ptr::null_mut()
            }
        }
    })
}

/// Returns the bundled locals query for the given language as a C string.
///
/// Returns null if no bundled query is available for the language.
/// The caller must free the returned pointer with `ts_pack_free_string`.
///
/// # Safety
///
/// `language` must be a valid null-terminated UTF-8 C string, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_get_locals_query(language: *const c_char) -> *mut c_char {
    ffi_guard!(ptr::null_mut(), {
        clear_last_error();
        if language.is_null() {
            set_last_error("language pointer is null");
            return ptr::null_mut();
        }
        // SAFETY: caller guarantees valid null-terminated string; null check above
        let lang_str = unsafe { CStr::from_ptr(language) };
        match lang_str.to_str() {
            Ok(s) => match tree_sitter_language_pack::get_locals_query(s) {
                Some(q) => CString::new(q).map(CString::into_raw).unwrap_or(ptr::null_mut()),
                None => ptr::null_mut(),
            },
            Err(e) => {
                set_last_error(&format!("invalid UTF-8 in language: {e}"));
                ptr::null_mut()
            }
        }
    })
}

/// Get the last error message, or null if no error occurred.
///
/// The returned pointer is valid until the next FFI call on the same thread.
/// The caller must NOT free this pointer.
///
/// # Safety
///
/// The returned pointer is only valid until the next FFI call on the same thread.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_last_error() -> *const c_char {
    LAST_ERROR.with(|e| match e.borrow().as_ref() {
        Some(c) => c.as_ptr(),
        None => ptr::null(),
    })
}

/// Clear the last error.
///
/// # Safety
///
/// This function is always safe to call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_clear_error() {
    clear_last_error();
}

/// Free a string that was returned by the FFI (e.g. from `ts_pack_language_name_at`).
///
/// Passing a null pointer is a safe no-op.
///
/// # Safety
///
/// `s` must be a pointer returned by an FFI function in this crate, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_free_string(s: *mut c_char) {
    ffi_guard!((), {
        if !s.is_null() {
            // SAFETY: the pointer was created by CString::into_raw in our code
            unsafe {
                drop(CString::from_raw(s));
            }
        }
    });
}

// ---------------------------------------------------------------------------
// Parsing functions
// ---------------------------------------------------------------------------

/// Parse a source string using the named language and return an opaque tree handle.
///
/// Returns null on error (check `ts_pack_last_error` for details).
/// The caller must free the tree with `ts_pack_tree_free`.
///
/// # Safety
///
/// `registry` must be a valid pointer returned by `ts_pack_registry_new`.
/// `name` and `source` must be valid null-terminated UTF-8 C strings.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_parse_string(
    registry: *const TsPackRegistry,
    name: *const c_char,
    source: *const c_char,
    source_len: usize,
) -> *mut TsPackTree {
    ffi_guard!(ptr::null_mut(), {
        clear_last_error();
        if registry.is_null() {
            set_last_error("registry pointer is null");
            return ptr::null_mut();
        }
        if name.is_null() {
            set_last_error("name pointer is null");
            return ptr::null_mut();
        }
        if source.is_null() {
            set_last_error("source pointer is null");
            return ptr::null_mut();
        }
        // SAFETY: caller guarantees valid pointer; null check above
        let reg = unsafe { &*registry };
        // SAFETY: caller guarantees valid null-terminated string; null check above
        let name_str = match unsafe { CStr::from_ptr(name) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_last_error(&format!("invalid UTF-8 in name: {e}"));
                return ptr::null_mut();
            }
        };
        // SAFETY: caller guarantees valid buffer with correct length
        let source_bytes = unsafe { std::slice::from_raw_parts(source as *const u8, source_len) };
        let lang = match reg.inner.get_language(name_str) {
            Ok(l) => l,
            Err(e) => {
                set_last_error(&e.to_string());
                return ptr::null_mut();
            }
        };
        let mut parser = tree_sitter::Parser::new();
        if let Err(e) = parser.set_language(&lang) {
            set_last_error(&format!("failed to set language: {e}"));
            return ptr::null_mut();
        }
        match parser.parse(source_bytes, None) {
            Some(tree) => Box::into_raw(Box::new(TsPackTree { inner: tree })),
            None => {
                set_last_error("parsing returned no tree");
                ptr::null_mut()
            }
        }
    })
}

/// Free a tree previously created with `ts_pack_parse_string`.
///
/// Passing a null pointer is a safe no-op.
///
/// # Safety
///
/// `tree` must be a pointer returned by `ts_pack_parse_string`, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_tree_free(tree: *mut TsPackTree) {
    ffi_guard!((), {
        if !tree.is_null() {
            // SAFETY: pointer was created by Box::into_raw in ts_pack_parse_string
            unsafe {
                drop(Box::from_raw(tree));
            }
        }
    });
}

/// Get the type name of the root node of the tree.
///
/// Returns a newly-allocated C string that the caller must free with
/// `ts_pack_free_string`. Returns null if the tree pointer is null.
///
/// # Safety
///
/// `tree` must be a valid pointer returned by `ts_pack_parse_string`, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_tree_root_node_type(tree: *const TsPackTree) -> *mut c_char {
    ffi_guard!(ptr::null_mut(), {
        clear_last_error();
        if tree.is_null() {
            set_last_error("tree pointer is null");
            return ptr::null_mut();
        }
        // SAFETY: caller guarantees valid pointer; null check above
        let t = unsafe { &*tree };
        let kind = t.inner.root_node().kind();
        match CString::new(kind) {
            Ok(c) => CString::into_raw(c),
            Err(e) => {
                set_last_error(&format!("node type contains null byte: {e}"));
                ptr::null_mut()
            }
        }
    })
}

/// Get the number of named children of the root node.
///
/// Returns 0 if the tree pointer is null.
///
/// # Safety
///
/// `tree` must be a valid pointer returned by `ts_pack_parse_string`, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_tree_root_child_count(tree: *const TsPackTree) -> u32 {
    ffi_guard!(0, {
        clear_last_error();
        if tree.is_null() {
            set_last_error("tree pointer is null");
            return 0;
        }
        // SAFETY: caller guarantees valid pointer; null check above
        let t = unsafe { &*tree };
        t.inner.root_node().named_child_count() as u32
    })
}

/// Check whether any node in the tree has the given type name.
///
/// Uses a depth-first traversal via TreeCursor.
///
/// # Safety
///
/// `tree` must be a valid pointer returned by `ts_pack_parse_string`, or null.
/// `node_type` must be a valid null-terminated UTF-8 C string, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_tree_contains_node_type(tree: *const TsPackTree, node_type: *const c_char) -> bool {
    ffi_guard!(false, {
        clear_last_error();
        if tree.is_null() {
            set_last_error("tree pointer is null");
            return false;
        }
        if node_type.is_null() {
            set_last_error("node_type pointer is null");
            return false;
        }
        // SAFETY: caller guarantees valid pointer; null check above
        let t = unsafe { &*tree };
        // SAFETY: caller guarantees valid null-terminated string; null check above
        let target = match unsafe { CStr::from_ptr(node_type) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_last_error(&format!("invalid UTF-8 in node_type: {e}"));
                return false;
            }
        };
        tree_sitter_language_pack::tree_contains_node_type(&t.inner, target)
    })
}

/// Check whether the tree contains any ERROR or MISSING nodes.
///
/// # Safety
///
/// `tree` must be a valid pointer returned by `ts_pack_parse_string`, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_tree_has_error_nodes(tree: *const TsPackTree) -> bool {
    ffi_guard!(false, {
        clear_last_error();
        if tree.is_null() {
            set_last_error("tree pointer is null");
            return false;
        }
        // SAFETY: caller guarantees valid pointer; null check above
        let t = unsafe { &*tree };
        tree_sitter_language_pack::tree_has_error_nodes(&t.inner)
    })
}

/// Return the S-expression representation of the tree.
///
/// Returns a newly-allocated C string that the caller must free with
/// `ts_pack_free_string`. Returns null if the tree pointer is null.
///
/// # Safety
///
/// `tree` must be a valid pointer returned by `ts_pack_parse_string`, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_tree_to_sexp(tree: *const TsPackTree) -> *mut c_char {
    ffi_guard!(ptr::null_mut(), {
        clear_last_error();
        if tree.is_null() {
            set_last_error("tree pointer is null");
            return ptr::null_mut();
        }
        // SAFETY: caller guarantees valid pointer; null check above
        let t = unsafe { &*tree };
        let sexp = tree_sitter_language_pack::tree_to_sexp(&t.inner);
        match CString::new(sexp) {
            Ok(c) => CString::into_raw(c),
            Err(e) => {
                set_last_error(&format!("sexp contains null byte: {e}"));
                ptr::null_mut()
            }
        }
    })
}

/// Return the count of ERROR and MISSING nodes in the tree.
///
/// Returns 0 if the tree pointer is null.
///
/// # Safety
///
/// `tree` must be a valid pointer returned by `ts_pack_parse_string`, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_tree_error_count(tree: *const TsPackTree) -> usize {
    ffi_guard!(0, {
        clear_last_error();
        if tree.is_null() {
            set_last_error("tree pointer is null");
            return 0;
        }
        // SAFETY: caller guarantees valid pointer; null check above
        let t = unsafe { &*tree };
        tree_sitter_language_pack::tree_error_count(&t.inner)
    })
}

// ---------------------------------------------------------------------------
// Process: unified API
// ---------------------------------------------------------------------------

/// Process source code and extract metadata + chunks as a JSON C string.
///
/// `config_json` is a null-terminated JSON string with fields:
/// - `language` (string, required): the language name
/// - `chunk_max_size` (number, optional): maximum chunk size in bytes (default: 1500)
///
/// Returns a newly-allocated C string that the caller must free with
/// `ts_pack_free_string`. Returns null on error (check `ts_pack_last_error`).
///
/// # Safety
///
/// `registry` must be a valid pointer returned by `ts_pack_registry_new`.
/// `source` must be a valid pointer to `source_len` bytes.
/// `config_json` must be a valid null-terminated UTF-8 C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_process(
    registry: *const TsPackRegistry,
    source: *const c_char,
    source_len: usize,
    config_json: *const c_char,
) -> *mut c_char {
    ffi_guard!(ptr::null_mut(), {
        clear_last_error();
        if registry.is_null() || source.is_null() || config_json.is_null() {
            set_last_error("null pointer argument");
            return ptr::null_mut();
        }
        // SAFETY: caller guarantees valid pointer; null check above
        let reg = unsafe { &*registry };
        // SAFETY: caller guarantees valid null-terminated string; null check above
        let config_str = match unsafe { CStr::from_ptr(config_json) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_last_error(&format!("invalid UTF-8 in config_json: {e}"));
                return ptr::null_mut();
            }
        };
        let config: serde_json::Value = match serde_json::from_str(config_str) {
            Ok(v) => v,
            Err(e) => {
                set_last_error(&format!("invalid JSON in config: {e}"));
                return ptr::null_mut();
            }
        };
        let core_config: tree_sitter_language_pack::ProcessConfig = match serde_json::from_value(config) {
            Ok(c) => c,
            Err(e) => {
                set_last_error(&format!("invalid config: {e}"));
                return ptr::null_mut();
            }
        };
        // SAFETY: caller guarantees valid buffer with correct length
        let source_bytes = unsafe { std::slice::from_raw_parts(source as *const u8, source_len) };
        let source_str = match std::str::from_utf8(source_bytes) {
            Ok(s) => s,
            Err(e) => {
                set_last_error(&format!("invalid UTF-8 in source: {e}"));
                return ptr::null_mut();
            }
        };
        match reg.inner.process(source_str, &core_config) {
            Ok(result) => match serde_json::to_string(&result) {
                Ok(json) => match CString::new(json) {
                    Ok(c) => CString::into_raw(c),
                    Err(e) => {
                        set_last_error(&format!("null byte in JSON: {e}"));
                        ptr::null_mut()
                    }
                },
                Err(e) => {
                    set_last_error(&format!("serialization failed: {e}"));
                    ptr::null_mut()
                }
            },
            Err(e) => {
                set_last_error(&e.to_string());
                ptr::null_mut()
            }
        }
    })
}

// ---------------------------------------------------------------------------
// Extraction API
// ---------------------------------------------------------------------------

/// Extract patterns from source code using a JSON extraction config.
///
/// `source` is a pointer to the source code bytes (not necessarily null-terminated).
/// `source_len` is the number of bytes in the source buffer.
/// `config_json` is a null-terminated JSON string with fields:
/// - `language` (string, required): the language name
/// - `patterns` (object, required): named patterns to extract
///
/// Returns a newly-allocated JSON C string with the extraction results.
/// The caller must free the returned pointer with `ts_pack_free_string`.
/// Returns null on error (check `ts_pack_last_error`).
///
/// # Safety
///
/// `source` must be a valid pointer to `source_len` bytes.
/// `config_json` must be a valid null-terminated UTF-8 C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_extract(
    source: *const c_char,
    source_len: usize,
    config_json: *const c_char,
) -> *mut c_char {
    ffi_guard!(ptr::null_mut(), {
        clear_last_error();
        if source.is_null() {
            set_last_error("source pointer is null");
            return ptr::null_mut();
        }
        if config_json.is_null() {
            set_last_error("config_json pointer is null");
            return ptr::null_mut();
        }
        // SAFETY: caller guarantees valid null-terminated string; null check above
        let config_str = match unsafe { CStr::from_ptr(config_json) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_last_error(&format!("invalid UTF-8 in config_json: {e}"));
                return ptr::null_mut();
            }
        };
        let config: serde_json::Value = match serde_json::from_str(config_str) {
            Ok(v) => v,
            Err(e) => {
                set_last_error(&format!("invalid JSON in config: {e}"));
                return ptr::null_mut();
            }
        };
        let extraction_config: tree_sitter_language_pack::ExtractionConfig = match serde_json::from_value(config) {
            Ok(c) => c,
            Err(e) => {
                set_last_error(&format!("invalid config: {e}"));
                return ptr::null_mut();
            }
        };
        // SAFETY: caller guarantees valid buffer with correct length
        let source_bytes = unsafe { std::slice::from_raw_parts(source as *const u8, source_len) };
        let source_str = match std::str::from_utf8(source_bytes) {
            Ok(s) => s,
            Err(e) => {
                set_last_error(&format!("invalid UTF-8 in source: {e}"));
                return ptr::null_mut();
            }
        };
        match tree_sitter_language_pack::extract_patterns(source_str, &extraction_config) {
            Ok(result) => match serde_json::to_string(&result) {
                Ok(json) => match CString::new(json) {
                    Ok(c) => CString::into_raw(c),
                    Err(e) => {
                        set_last_error(&format!("null byte in JSON: {e}"));
                        ptr::null_mut()
                    }
                },
                Err(e) => {
                    set_last_error(&format!("serialization failed: {e}"));
                    ptr::null_mut()
                }
            },
            Err(e) => {
                set_last_error(&e.to_string());
                ptr::null_mut()
            }
        }
    })
}

/// Validate extraction patterns without running them.
///
/// `config_json` is a null-terminated JSON string with the same shape as for
/// `ts_pack_extract` (language + patterns).
///
/// Returns a newly-allocated JSON C string with validation results.
/// The caller must free the returned pointer with `ts_pack_free_string`.
/// Returns null on error (check `ts_pack_last_error`).
///
/// # Safety
///
/// `config_json` must be a valid null-terminated UTF-8 C string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_validate_extraction(config_json: *const c_char) -> *mut c_char {
    ffi_guard!(ptr::null_mut(), {
        clear_last_error();
        if config_json.is_null() {
            set_last_error("config_json pointer is null");
            return ptr::null_mut();
        }
        // SAFETY: caller guarantees valid null-terminated string; null check above
        let config_str = match unsafe { CStr::from_ptr(config_json) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_last_error(&format!("invalid UTF-8 in config_json: {e}"));
                return ptr::null_mut();
            }
        };
        let config: serde_json::Value = match serde_json::from_str(config_str) {
            Ok(v) => v,
            Err(e) => {
                set_last_error(&format!("invalid JSON in config: {e}"));
                return ptr::null_mut();
            }
        };
        let extraction_config: tree_sitter_language_pack::ExtractionConfig = match serde_json::from_value(config) {
            Ok(c) => c,
            Err(e) => {
                set_last_error(&format!("invalid config: {e}"));
                return ptr::null_mut();
            }
        };
        match tree_sitter_language_pack::validate_extraction(&extraction_config) {
            Ok(result) => match serde_json::to_string(&result) {
                Ok(json) => match CString::new(json) {
                    Ok(c) => CString::into_raw(c),
                    Err(e) => {
                        set_last_error(&format!("null byte in JSON: {e}"));
                        ptr::null_mut()
                    }
                },
                Err(e) => {
                    set_last_error(&format!("serialization failed: {e}"));
                    ptr::null_mut()
                }
            },
            Err(e) => {
                set_last_error(&e.to_string());
                ptr::null_mut()
            }
        }
    })
}

// ---------------------------------------------------------------------------
// Download API
// ---------------------------------------------------------------------------

/// Initialize the language pack with configuration.
///
/// `config_json` is a null-terminated JSON string with optional fields:
/// - `cache_dir` (string): override default cache directory
/// - `languages` (array of strings): languages to pre-download
/// - `groups` (array of strings): language groups to pre-download
///
/// Returns 0 on success, -1 on error (check `ts_pack_last_error`).
///
/// # Safety
///
/// `config_json` must be a valid null-terminated UTF-8 C string, or null.
#[cfg(feature = "download")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_init(config_json: *const c_char) -> i32 {
    ffi_guard!(-1, {
        clear_last_error();
        let config_str = if config_json.is_null() {
            "{}"
        } else {
            // SAFETY: caller guarantees valid null-terminated string; null check above
            match unsafe { CStr::from_ptr(config_json) }.to_str() {
                Ok(s) => s,
                Err(e) => {
                    set_last_error(&format!("invalid UTF-8 in config_json: {e}"));
                    return -1;
                }
            }
        };
        let config: PackConfig = match serde_json::from_str(config_str) {
            Ok(c) => c,
            Err(e) => {
                set_last_error(&format!("invalid JSON in config: {e}"));
                return -1;
            }
        };
        match tree_sitter_language_pack::init(&config) {
            Ok(()) => 0,
            Err(e) => {
                set_last_error(&e.to_string());
                -1
            }
        }
    })
}

/// Configure the language pack without downloading.
///
/// `config_json` is a null-terminated JSON string with optional fields:
/// - `cache_dir` (string): override default cache directory
///
/// Returns 0 on success, -1 on error (check `ts_pack_last_error`).
///
/// # Safety
///
/// `config_json` must be a valid null-terminated UTF-8 C string, or null.
#[cfg(feature = "download")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_configure(config_json: *const c_char) -> i32 {
    ffi_guard!(-1, {
        clear_last_error();
        let config_str = if config_json.is_null() {
            "{}"
        } else {
            // SAFETY: caller guarantees valid null-terminated string; null check above
            match unsafe { CStr::from_ptr(config_json) }.to_str() {
                Ok(s) => s,
                Err(e) => {
                    set_last_error(&format!("invalid UTF-8 in config_json: {e}"));
                    return -1;
                }
            }
        };
        let config: PackConfig = match serde_json::from_str(config_str) {
            Ok(c) => c,
            Err(e) => {
                set_last_error(&format!("invalid JSON in config: {e}"));
                return -1;
            }
        };
        match tree_sitter_language_pack::configure(&config) {
            Ok(()) => 0,
            Err(e) => {
                set_last_error(&e.to_string());
                -1
            }
        }
    })
}

/// Download specific languages to the cache.
///
/// `names` is an array of pointers to null-terminated language name strings.
/// `count` is the number of strings in the array.
///
/// Returns the number of newly downloaded languages on success, or -1 on error.
///
/// # Safety
///
/// `names` must be a valid array of `count` pointers to null-terminated UTF-8 C strings.
#[cfg(feature = "download")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_download(names: *const *const c_char, count: usize) -> i32 {
    ffi_guard!(-1, {
        clear_last_error();
        if names.is_null() && count > 0 {
            set_last_error("names array is null but count > 0");
            return -1;
        }
        let mut lang_names: Vec<&str> = Vec::with_capacity(count);
        // SAFETY: caller guarantees valid array of pointers
        let names_arr = unsafe { std::slice::from_raw_parts(names, count) };
        for name_ptr in names_arr {
            if name_ptr.is_null() {
                set_last_error("null pointer in names array");
                return -1;
            }
            // SAFETY: caller guarantees valid null-terminated C string
            match unsafe { CStr::from_ptr(*name_ptr) }.to_str() {
                Ok(s) => lang_names.push(s),
                Err(e) => {
                    set_last_error(&format!("invalid UTF-8 in language name: {e}"));
                    return -1;
                }
            }
        }
        match tree_sitter_language_pack::download(&lang_names) {
            Ok(n) => n as i32,
            Err(e) => {
                set_last_error(&e.to_string());
                -1
            }
        }
    })
}

/// Download all available languages from the remote manifest.
///
/// Returns the number of newly downloaded languages on success, or -1 on error.
///
/// # Safety
///
/// This function is safe to call; it does not take any unsafe parameters.
#[cfg(feature = "download")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_download_all() -> i32 {
    ffi_guard!(-1, {
        clear_last_error();
        match tree_sitter_language_pack::download_all() {
            Ok(n) => n as i32,
            Err(e) => {
                set_last_error(&e.to_string());
                -1
            }
        }
    })
}

/// Get all language names available in the remote manifest.
///
/// Returns a newly-allocated array of language name strings. The caller must
/// free the array with `ts_pack_free_string_array`, and the individual strings
/// with `ts_pack_free_string`.
///
/// Sets `out_count` to the number of languages in the returned array.
/// Returns null on error (check `ts_pack_last_error`).
///
/// # Safety
///
/// `out_count` must be a valid, non-null pointer to a `usize` that the caller owns.
#[cfg(feature = "download")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_manifest_languages(out_count: *mut usize) -> *const *const c_char {
    ffi_guard!(ptr::null(), {
        clear_last_error();
        if out_count.is_null() {
            set_last_error("out_count pointer is null");
            return ptr::null();
        }
        match tree_sitter_language_pack::manifest_languages() {
            Ok(langs) => {
                let c_strings: Vec<*const c_char> = langs
                    .into_iter()
                    .filter_map(|name| CString::new(name).ok())
                    .map(|c| CString::into_raw(c) as *const c_char)
                    .collect();
                let count = c_strings.len();
                // SAFETY: caller owns the returned pointer and must free with ts_pack_free_string_array
                unsafe {
                    *out_count = count;
                }
                Box::into_raw(c_strings.into_boxed_slice()) as *const *const c_char
            }
            Err(e) => {
                set_last_error(&e.to_string());
                ptr::null()
            }
        }
    })
}

/// Get all languages that are already downloaded and cached locally.
///
/// Returns a newly-allocated array of language name strings. The caller must
/// free the array with `ts_pack_free_string_array`, and the individual strings
/// with `ts_pack_free_string`.
///
/// Sets `out_count` to the number of languages in the returned array.
/// Returns null if the count pointer is null, but never fails otherwise.
///
/// # Safety
///
/// `out_count` must be a valid, non-null pointer to a `usize` that the caller owns.
#[cfg(feature = "download")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_downloaded_languages(out_count: *mut usize) -> *const *const c_char {
    ffi_guard!(ptr::null(), {
        clear_last_error();
        if out_count.is_null() {
            set_last_error("out_count pointer is null");
            return ptr::null();
        }
        let langs = tree_sitter_language_pack::downloaded_languages();
        let c_strings: Vec<*const c_char> = langs
            .into_iter()
            .filter_map(|name| CString::new(name).ok())
            .map(|c| CString::into_raw(c) as *const c_char)
            .collect();
        let count = c_strings.len();
        // SAFETY: caller owns the returned pointer and must free with ts_pack_free_string_array
        unsafe {
            *out_count = count;
        }
        Box::into_raw(c_strings.into_boxed_slice()) as *const *const c_char
    })
}

/// Delete all cached parser shared libraries.
///
/// Returns 0 on success, -1 on error (check `ts_pack_last_error`).
///
/// # Safety
///
/// This function is safe to call; it does not take any unsafe parameters.
#[cfg(feature = "download")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_clean_cache() -> i32 {
    ffi_guard!(-1, {
        clear_last_error();
        match tree_sitter_language_pack::clean_cache() {
            Ok(()) => 0,
            Err(e) => {
                set_last_error(&e.to_string());
                -1
            }
        }
    })
}

/// Get the effective cache directory path as a C string.
///
/// Returns a newly-allocated C string that the caller must free with
/// `ts_pack_free_string`. Returns null on error (check `ts_pack_last_error`).
///
/// # Safety
///
/// This function is safe to call; it does not take any unsafe parameters.
#[cfg(feature = "download")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_cache_dir() -> *mut c_char {
    ffi_guard!(ptr::null_mut(), {
        clear_last_error();
        match tree_sitter_language_pack::cache_dir() {
            Ok(path) => match CString::new(path.to_string_lossy().to_string()) {
                Ok(c) => CString::into_raw(c),
                Err(e) => {
                    set_last_error(&format!("path contains null byte: {e}"));
                    ptr::null_mut()
                }
            },
            Err(e) => {
                set_last_error(&e.to_string());
                ptr::null_mut()
            }
        }
    })
}

/// Free a string array that was returned by the FFI (e.g. from `ts_pack_manifest_languages`).
///
/// Passing a null pointer is a safe no-op.
/// This function only frees the array wrapper, not the individual strings.
/// Use `ts_pack_free_string` on each individual string before calling this.
///
/// # Safety
///
/// `arr` must be a pointer returned by an FFI function in this crate that returns
/// an array, or null. Individual strings must be freed first with `ts_pack_free_string`.
#[cfg(feature = "download")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ts_pack_free_string_array(arr: *mut *const c_char) {
    ffi_guard!((), {
        if !arr.is_null() {
            // SAFETY: the pointer was created by Box::into_raw in our code
            unsafe {
                drop(Box::from_raw(arr));
            }
        }
    });
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_registry_create_and_free() {
        unsafe {
            let reg = ts_pack_registry_new();
            assert!(!reg.is_null(), "registry should not be null");
            ts_pack_registry_free(reg);
        }
    }

    #[test]
    fn test_free_null_registry_is_safe() {
        unsafe {
            ts_pack_registry_free(ptr::null_mut());
        }
    }

    #[test]
    fn test_language_count() {
        unsafe {
            let reg = ts_pack_registry_new();
            let count = ts_pack_language_count(reg);
            assert!(count > 0, "should have at least one language");
            ts_pack_registry_free(reg);
        }
    }

    #[test]
    fn test_language_name_at() {
        unsafe {
            let reg = ts_pack_registry_new();
            let count = ts_pack_language_count(reg);
            assert!(count > 0);

            // Valid index
            let name_ptr = ts_pack_language_name_at(reg, 0);
            assert!(!name_ptr.is_null());
            let name = CStr::from_ptr(name_ptr).to_str().expect("valid UTF-8");
            assert!(!name.is_empty());
            ts_pack_free_string(name_ptr as *mut c_char);

            // Out of bounds
            let bad = ts_pack_language_name_at(reg, count + 100);
            assert!(bad.is_null());
            assert!(!ts_pack_last_error().is_null());

            ts_pack_registry_free(reg);
        }
    }

    #[test]
    fn test_get_language() {
        unsafe {
            let reg = ts_pack_registry_new();

            // Get the first available language name and try loading it
            let count = ts_pack_language_count(reg);
            assert!(count > 0);
            let name_ptr = ts_pack_language_name_at(reg, 0);
            assert!(!name_ptr.is_null());

            let lang = ts_pack_get_language(reg, name_ptr);
            assert!(!lang.is_null(), "should load first available language; error: {:?}", {
                let err = ts_pack_last_error();
                if err.is_null() {
                    "none".to_string()
                } else {
                    CStr::from_ptr(err).to_str().unwrap_or("?").to_string()
                }
            });

            ts_pack_free_string(name_ptr as *mut c_char);
            ts_pack_registry_free(reg);
        }
    }

    #[test]
    fn test_has_language() {
        unsafe {
            let reg = ts_pack_registry_new();

            let name_ptr = ts_pack_language_name_at(reg, 0);
            assert!(!name_ptr.is_null());
            assert!(ts_pack_has_language(reg, name_ptr));
            ts_pack_free_string(name_ptr as *mut c_char);

            let bad = CString::new("nonexistent_language_xyz_42").unwrap();
            assert!(!ts_pack_has_language(reg, bad.as_ptr()));

            ts_pack_registry_free(reg);
        }
    }

    #[test]
    fn test_error_on_unknown_language() {
        unsafe {
            let reg = ts_pack_registry_new();
            let name = CString::new("nonexistent_language_xyz_42").unwrap();
            let lang = ts_pack_get_language(reg, name.as_ptr());
            assert!(lang.is_null());

            let err = ts_pack_last_error();
            assert!(!err.is_null());
            let msg = CStr::from_ptr(err).to_str().expect("valid UTF-8");
            assert!(
                msg.contains("not found") || msg.contains("nonexistent"),
                "error message should mention the issue: {msg}"
            );

            ts_pack_registry_free(reg);
        }
    }

    #[test]
    fn test_null_inputs() {
        unsafe {
            // Null registry
            assert!(ts_pack_get_language(ptr::null(), ptr::null()).is_null());
            assert_eq!(ts_pack_language_count(ptr::null()), 0);
            assert!(ts_pack_language_name_at(ptr::null(), 0).is_null());
            assert!(!ts_pack_has_language(ptr::null(), ptr::null()));

            // Null name
            let reg = ts_pack_registry_new();
            assert!(ts_pack_get_language(reg, ptr::null()).is_null());
            assert!(!ts_pack_has_language(reg, ptr::null()));
            ts_pack_registry_free(reg);
        }
    }

    #[test]
    fn test_clear_error() {
        unsafe {
            // Trigger an error
            let name = CString::new("nonexistent").unwrap();
            let reg = ts_pack_registry_new();
            ts_pack_get_language(reg, name.as_ptr());
            assert!(!ts_pack_last_error().is_null());

            // Clear it
            ts_pack_clear_error();
            assert!(ts_pack_last_error().is_null());

            ts_pack_registry_free(reg);
        }
    }

    #[test]
    fn test_free_null_string_is_safe() {
        unsafe {
            ts_pack_free_string(ptr::null_mut());
        }
    }

    #[test]
    fn test_parse_string() {
        unsafe {
            let reg = ts_pack_registry_new();
            let name = CString::new("python").unwrap();
            let source = b"def hello(): pass";
            let tree = ts_pack_parse_string(reg, name.as_ptr(), source.as_ptr() as *const c_char, source.len());
            assert!(!tree.is_null(), "tree should not be null; error: {:?}", {
                let err = ts_pack_last_error();
                if err.is_null() {
                    "none".to_string()
                } else {
                    CStr::from_ptr(err).to_str().unwrap_or("?").to_string()
                }
            });

            // Check root node type
            let root_type = ts_pack_tree_root_node_type(tree);
            assert!(!root_type.is_null());
            let root_str = CStr::from_ptr(root_type).to_str().unwrap();
            assert_eq!(root_str, "module");
            ts_pack_free_string(root_type);

            // Check child count
            let count = ts_pack_tree_root_child_count(tree);
            assert!(count >= 1, "should have at least 1 child");

            // Check contains node type
            let func_def = CString::new("function_definition").unwrap();
            assert!(ts_pack_tree_contains_node_type(tree, func_def.as_ptr()));

            let bogus = CString::new("nonexistent_node_xyz").unwrap();
            assert!(!ts_pack_tree_contains_node_type(tree, bogus.as_ptr()));

            // Check no error nodes in valid code
            assert!(!ts_pack_tree_has_error_nodes(tree));

            ts_pack_tree_free(tree);
            ts_pack_registry_free(reg);
        }
    }

    #[test]
    fn test_parse_string_with_errors() {
        unsafe {
            let reg = ts_pack_registry_new();
            let name = CString::new("python").unwrap();
            let source = b"def (broken syntax @@@ !!!";
            let tree = ts_pack_parse_string(reg, name.as_ptr(), source.as_ptr() as *const c_char, source.len());
            assert!(!tree.is_null());

            assert!(ts_pack_tree_has_error_nodes(tree));

            ts_pack_tree_free(tree);
            ts_pack_registry_free(reg);
        }
    }

    #[test]
    fn test_parse_null_inputs() {
        unsafe {
            let reg = ts_pack_registry_new();
            let name = CString::new("python").unwrap();

            // Null registry
            assert!(ts_pack_parse_string(ptr::null(), name.as_ptr(), name.as_ptr(), 0).is_null());
            // Null name
            assert!(ts_pack_parse_string(reg, ptr::null(), name.as_ptr(), 0).is_null());
            // Null source
            assert!(ts_pack_parse_string(reg, name.as_ptr(), ptr::null(), 0).is_null());

            // Null tree for inspection functions
            assert!(ts_pack_tree_root_node_type(ptr::null()).is_null());
            assert_eq!(ts_pack_tree_root_child_count(ptr::null()), 0);
            assert!(!ts_pack_tree_contains_node_type(ptr::null(), name.as_ptr()));
            assert!(!ts_pack_tree_has_error_nodes(ptr::null()));

            // Free null tree is safe
            ts_pack_tree_free(ptr::null_mut());

            ts_pack_registry_free(reg);
        }
    }
}
