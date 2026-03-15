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
use ts_pack_core::LanguageRegistry;

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
        let reg = unsafe { &*registry };
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
        let reg = unsafe { &*registry };
        let name_str = match unsafe { CStr::from_ptr(name) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_last_error(&format!("invalid UTF-8 in name: {e}"));
                return ptr::null_mut();
            }
        };
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
        let t = unsafe { &*tree };
        let target = match unsafe { CStr::from_ptr(node_type) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_last_error(&format!("invalid UTF-8 in node_type: {e}"));
                return false;
            }
        };
        ts_pack_core::tree_contains_node_type(&t.inner, target)
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
        let t = unsafe { &*tree };
        ts_pack_core::tree_has_error_nodes(&t.inner)
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
        let t = unsafe { &*tree };
        let sexp = ts_pack_core::tree_to_sexp(&t.inner);
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
        let t = unsafe { &*tree };
        ts_pack_core::tree_error_count(&t.inner)
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
        let reg = unsafe { &*registry };
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
        let core_config: ts_pack_core::ProcessConfig = match serde_json::from_value(config) {
            Ok(c) => c,
            Err(e) => {
                set_last_error(&format!("invalid config: {e}"));
                return ptr::null_mut();
            }
        };
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
