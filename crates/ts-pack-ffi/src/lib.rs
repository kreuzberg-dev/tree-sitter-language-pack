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
// Opaque handle
// ---------------------------------------------------------------------------

/// Opaque handle to a language registry.
/// Created with `ts_pack_registry_new` and freed with `ts_pack_registry_free`.
pub struct TsPackRegistry {
    inner: LanguageRegistry,
    /// Cached sorted list of language names kept in sync with the registry.
    cached_names: Vec<CString>,
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
}
