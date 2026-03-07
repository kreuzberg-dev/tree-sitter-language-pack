use pyo3::prelude::*;
use pyo3::types::PyList;

/// The PyCapsule name used by the tree-sitter Python package.
const CAPSULE_NAME: &std::ffi::CStr = c"tree_sitter.Language";

/// Returns a PyCapsule wrapping the raw TSLanguage pointer.
/// The capsule name is "tree_sitter.Language\0" for compatibility with the
/// tree-sitter Python package.
#[pyfunction]
fn get_binding(py: Python<'_>, name: &str) -> PyResult<PyObject> {
    let language =
        ts_pack_core::get_language(name).map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{e}")))?;

    // SAFETY: tree_sitter::Language is #[repr(transparent)] over *const ffi::TSLanguage,
    // so transmuting to extract the inner pointer is sound. The pointer remains valid
    // because the static registry keeps compiled parsers alive for the program lifetime.
    let raw_ptr: *const tree_sitter::ffi::TSLanguage = unsafe { std::mem::transmute(language) };

    // SAFETY: PyCapsule_New creates a new PyCapsule. raw_ptr is valid for the
    // duration of the program (static registry keeps parsers alive).
    let capsule_ptr = unsafe { pyo3::ffi::PyCapsule_New(raw_ptr as *mut _, CAPSULE_NAME.as_ptr(), None) };

    if capsule_ptr.is_null() {
        return Err(pyo3::exceptions::PyRuntimeError::new_err(
            "Failed to create PyCapsule for language binding",
        ));
    }

    // SAFETY: capsule_ptr is a valid, non-null Python object we just created.
    Ok(unsafe { PyObject::from_owned_ptr(py, capsule_ptr) })
}

/// Returns a tree_sitter.Language instance for the given language name.
#[pyfunction]
fn get_language(py: Python<'_>, name: &str) -> PyResult<PyObject> {
    let capsule = get_binding(py, name)?;

    let tree_sitter_mod = py.import("tree_sitter")?;
    let language_class = tree_sitter_mod.getattr("Language")?;
    let language = language_class.call1((capsule,))?;

    Ok(language.unbind())
}

/// Returns a tree_sitter.Parser pre-configured for the given language.
#[pyfunction]
fn get_parser(py: Python<'_>, name: &str) -> PyResult<PyObject> {
    let language = get_language(py, name)?;

    let tree_sitter_mod = py.import("tree_sitter")?;
    let parser_class = tree_sitter_mod.getattr("Parser")?;
    let parser = parser_class.call1((language,))?;

    Ok(parser.unbind())
}

/// Returns a list of all available language names.
#[pyfunction]
fn available_languages(py: Python<'_>) -> PyResult<PyObject> {
    let langs = ts_pack_core::available_languages();
    let py_list = PyList::new(py, &langs)?;
    Ok(py_list.into_any().unbind())
}

/// Checks if a language is available.
#[pyfunction]
fn has_language(name: &str) -> bool {
    ts_pack_core::has_language(name)
}

#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_binding, m)?)?;
    m.add_function(wrap_pyfunction!(get_language, m)?)?;
    m.add_function(wrap_pyfunction!(get_parser, m)?)?;
    m.add_function(wrap_pyfunction!(available_languages, m)?)?;
    m.add_function(wrap_pyfunction!(has_language, m)?)?;
    Ok(())
}
