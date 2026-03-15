use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::sync::Mutex;

pyo3::create_exception!(
    tree_sitter_language_pack,
    LanguageNotFoundError,
    pyo3::exceptions::PyValueError
);

pyo3::create_exception!(tree_sitter_language_pack, ParseError, pyo3::exceptions::PyRuntimeError);

pyo3::create_exception!(tree_sitter_language_pack, QueryError, pyo3::exceptions::PyValueError);

/// The PyCapsule name used by the tree-sitter Python package.
const CAPSULE_NAME: &std::ffi::CStr = c"tree_sitter.Language";

// ---------------------------------------------------------------------------
// Language discovery
// ---------------------------------------------------------------------------

/// Returns a PyCapsule wrapping the raw TSLanguage pointer.
/// The capsule name is "tree_sitter.Language\0" for compatibility with the
/// tree-sitter Python package.
#[pyfunction]
fn get_binding(py: Python<'_>, name: &str) -> PyResult<Py<PyAny>> {
    let language = ts_pack_core::get_language(name).map_err(|e| LanguageNotFoundError::new_err(format!("{e}")))?;

    // Extract the raw pointer - valid for program lifetime (static registry).
    let raw_ptr: *const tree_sitter::ffi::TSLanguage = language.into_raw();

    // SAFETY: PyCapsule_New creates a new PyCapsule. raw_ptr is valid for the
    // duration of the program (static registry keeps parsers alive).
    let capsule_ptr = unsafe { pyo3::ffi::PyCapsule_New(raw_ptr as *mut _, CAPSULE_NAME.as_ptr(), None) };

    if capsule_ptr.is_null() {
        return Err(pyo3::exceptions::PyRuntimeError::new_err(
            "Failed to create PyCapsule for language binding",
        ));
    }

    // SAFETY: capsule_ptr is a valid, non-null Python object we just created.
    Ok(unsafe { Bound::from_owned_ptr(py, capsule_ptr) }.unbind())
}

/// Returns a tree_sitter.Language instance for the given language name.
#[pyfunction]
fn get_language(py: Python<'_>, name: &str) -> PyResult<Py<PyAny>> {
    let capsule = get_binding(py, name)?;

    let tree_sitter_mod = py.import("tree_sitter")?;
    let language_class = tree_sitter_mod.getattr("Language")?;
    let language = language_class.call1((capsule,))?;

    Ok(language.unbind())
}

/// Returns a tree_sitter.Parser pre-configured for the given language.
#[pyfunction]
fn get_parser(py: Python<'_>, name: &str) -> PyResult<Py<PyAny>> {
    let language = get_language(py, name)?;

    let tree_sitter_mod = py.import("tree_sitter")?;
    let parser_class = tree_sitter_mod.getattr("Parser")?;
    let parser = parser_class.call1((language,))?;

    Ok(parser.unbind())
}

/// Returns a list of all available language names.
#[pyfunction]
fn available_languages(py: Python<'_>) -> PyResult<Py<PyAny>> {
    let langs = ts_pack_core::available_languages();
    let py_list = PyList::new(py, &langs)?;
    Ok(py_list.into_any().unbind())
}

/// Checks if a language is available.
#[pyfunction]
fn has_language(name: &str) -> bool {
    ts_pack_core::has_language(name)
}

/// Returns the number of available languages.
#[pyfunction]
fn language_count() -> usize {
    ts_pack_core::language_count()
}

// ---------------------------------------------------------------------------
// Opaque tree handle
// ---------------------------------------------------------------------------

/// Wraps a tree-sitter Tree for safe sharing across the Python boundary.
#[pyclass]
struct TreeHandle {
    inner: Mutex<tree_sitter::Tree>,
    source: Vec<u8>,
}

#[pymethods]
impl TreeHandle {
    /// Returns the type name of the root node.
    fn root_node_type(&self) -> PyResult<String> {
        let guard = self.inner.lock().map_err(|_| ParseError::new_err("lock poisoned"))?;
        Ok(guard.root_node().kind().to_string())
    }

    /// Returns the number of named children of the root node.
    fn root_child_count(&self) -> PyResult<u32> {
        let guard = self.inner.lock().map_err(|_| ParseError::new_err("lock poisoned"))?;
        Ok(guard.root_node().named_child_count() as u32)
    }

    /// Check whether any node in the tree has the given type name.
    fn contains_node_type(&self, node_type: &str) -> PyResult<bool> {
        let guard = self.inner.lock().map_err(|_| ParseError::new_err("lock poisoned"))?;
        Ok(ts_pack_core::tree_contains_node_type(&guard, node_type))
    }

    /// Check whether the tree contains any ERROR or MISSING nodes.
    fn has_error_nodes(&self) -> PyResult<bool> {
        let guard = self.inner.lock().map_err(|_| ParseError::new_err("lock poisoned"))?;
        Ok(ts_pack_core::tree_has_error_nodes(&guard))
    }

    /// Returns the S-expression representation of the tree.
    fn to_sexp(&self) -> PyResult<String> {
        let guard = self.inner.lock().map_err(|_| ParseError::new_err("lock poisoned"))?;
        Ok(ts_pack_core::tree_to_sexp(&guard))
    }

    /// Returns the count of ERROR and MISSING nodes in the tree.
    fn error_count(&self) -> PyResult<usize> {
        let guard = self.inner.lock().map_err(|_| ParseError::new_err("lock poisoned"))?;
        Ok(ts_pack_core::tree_error_count(&guard))
    }

    /// Returns information about the root node as a dict.
    fn root_node_info(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let guard = self.inner.lock().map_err(|_| ParseError::new_err("lock poisoned"))?;
        let info = ts_pack_core::root_node_info(&guard);
        node_info_to_dict(py, &info)
    }

    /// Finds all nodes matching the given type and returns their info as a list of dicts.
    fn find_nodes_by_type(&self, py: Python<'_>, node_type: &str) -> PyResult<Py<PyAny>> {
        let guard = self.inner.lock().map_err(|_| ParseError::new_err("lock poisoned"))?;
        let nodes = ts_pack_core::find_nodes_by_type(&guard, node_type);
        let py_list: Vec<Py<PyAny>> = nodes
            .iter()
            .map(|info| node_info_to_dict(py, info))
            .collect::<PyResult<_>>()?;
        let list = PyList::new(py, &py_list)?;
        Ok(list.into_any().unbind())
    }

    /// Returns info for all named children of the root node.
    fn named_children_info(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let guard = self.inner.lock().map_err(|_| ParseError::new_err("lock poisoned"))?;
        let nodes = ts_pack_core::named_children_info(&guard);
        let py_list: Vec<Py<PyAny>> = nodes
            .iter()
            .map(|info| node_info_to_dict(py, info))
            .collect::<PyResult<_>>()?;
        let list = PyList::new(py, &py_list)?;
        Ok(list.into_any().unbind())
    }

    /// Extracts source text for a node given its start_byte and end_byte.
    fn extract_text(&self, start_byte: usize, end_byte: usize) -> PyResult<String> {
        let info = ts_pack_core::NodeInfo {
            kind: String::new(),
            is_named: false,
            start_byte,
            end_byte,
            start_row: 0,
            start_col: 0,
            end_row: 0,
            end_col: 0,
            named_child_count: 0,
            is_error: false,
            is_missing: false,
        };
        ts_pack_core::extract_text(&self.source, &info)
            .map(|s| s.to_string())
            .map_err(|e| ParseError::new_err(format!("{e}")))
    }

    /// Runs a tree-sitter query and returns matches as a list of dicts.
    fn run_query(&self, py: Python<'_>, language: &str, query_source: &str) -> PyResult<Py<PyAny>> {
        let guard = self.inner.lock().map_err(|_| ParseError::new_err("lock poisoned"))?;
        let matches = ts_pack_core::run_query(&guard, language, query_source, &self.source)
            .map_err(|e| QueryError::new_err(format!("{e}")))?;

        let py_matches: Vec<Py<PyAny>> = matches
            .iter()
            .map(|m| query_match_to_dict(py, m))
            .collect::<PyResult<_>>()?;
        let list = PyList::new(py, &py_matches)?;
        Ok(list.into_any().unbind())
    }
}

// ---------------------------------------------------------------------------
// Parsing
// ---------------------------------------------------------------------------

/// Parse source code with the named language, returning a TreeHandle.
#[pyfunction]
fn parse_string(language: &str, source: &str) -> PyResult<TreeHandle> {
    let source_bytes = source.as_bytes();
    let tree = ts_pack_core::parse_string(language, source_bytes).map_err(|e| ParseError::new_err(format!("{e}")))?;
    Ok(TreeHandle {
        inner: Mutex::new(tree),
        source: source_bytes.to_vec(),
    })
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn node_info_to_dict(py: Python<'_>, info: &ts_pack_core::NodeInfo) -> PyResult<Py<PyAny>> {
    let dict = PyDict::new(py);
    dict.set_item("kind", &info.kind)?;
    dict.set_item("is_named", info.is_named)?;
    dict.set_item("start_byte", info.start_byte)?;
    dict.set_item("end_byte", info.end_byte)?;
    dict.set_item("start_row", info.start_row)?;
    dict.set_item("start_column", info.start_col)?;
    dict.set_item("end_row", info.end_row)?;
    dict.set_item("end_column", info.end_col)?;
    dict.set_item("named_child_count", info.named_child_count)?;
    dict.set_item("is_error", info.is_error)?;
    dict.set_item("is_missing", info.is_missing)?;
    Ok(dict.into_any().unbind())
}

fn query_match_to_dict(py: Python<'_>, qm: &ts_pack_core::QueryMatch) -> PyResult<Py<PyAny>> {
    let dict = PyDict::new(py);
    dict.set_item("pattern_index", qm.pattern_index)?;

    let captures: Vec<Py<PyAny>> = qm
        .captures
        .iter()
        .map(|(name, info)| {
            let capture_dict = PyDict::new(py);
            capture_dict.set_item("name", name)?;
            capture_dict.set_item("node", node_info_to_dict(py, info)?)?;
            Ok(capture_dict.into_any().unbind())
        })
        .collect::<PyResult<_>>()?;

    let captures_list = PyList::new(py, &captures)?;
    dict.set_item("captures", captures_list)?;
    Ok(dict.into_any().unbind())
}

// ---------------------------------------------------------------------------
// ProcessConfig pyclass
// ---------------------------------------------------------------------------

/// Configuration for the `process` function.
///
/// Controls which analysis features are enabled and chunking behavior.
#[pyclass(from_py_object)]
#[derive(Clone)]
struct ProcessConfig {
    #[pyo3(get, set)]
    language: String,
    #[pyo3(get, set)]
    structure: bool,
    #[pyo3(get, set)]
    imports: bool,
    #[pyo3(get, set)]
    exports: bool,
    #[pyo3(get, set)]
    comments: bool,
    #[pyo3(get, set)]
    docstrings: bool,
    #[pyo3(get, set)]
    symbols: bool,
    #[pyo3(get, set)]
    diagnostics: bool,
    #[pyo3(get, set)]
    chunk_max_size: Option<usize>,
}

#[pymethods]
impl ProcessConfig {
    #[new]
    #[pyo3(signature = (
        language,
        *,
        structure = true,
        imports = true,
        exports = true,
        comments = true,
        docstrings = true,
        symbols = true,
        diagnostics = true,
        chunk_max_size = None,
    ))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        language: String,
        structure: bool,
        imports: bool,
        exports: bool,
        comments: bool,
        docstrings: bool,
        symbols: bool,
        diagnostics: bool,
        chunk_max_size: Option<usize>,
    ) -> Self {
        Self {
            language,
            structure,
            imports,
            exports,
            comments,
            docstrings,
            symbols,
            diagnostics,
            chunk_max_size,
        }
    }

    /// Create a config with all features enabled and no chunking.
    #[staticmethod]
    fn all(language: String) -> Self {
        Self {
            language,
            structure: true,
            imports: true,
            exports: true,
            comments: true,
            docstrings: true,
            symbols: true,
            diagnostics: true,
            chunk_max_size: None,
        }
    }

    /// Create a config with only language and metrics (minimal extraction).
    #[staticmethod]
    fn minimal(language: String) -> Self {
        Self {
            language,
            structure: false,
            imports: false,
            exports: false,
            comments: false,
            docstrings: false,
            symbols: false,
            diagnostics: false,
            chunk_max_size: None,
        }
    }
}

impl From<&ProcessConfig> for ts_pack_core::ProcessConfig {
    fn from(py_config: &ProcessConfig) -> Self {
        Self {
            language: py_config.language.clone(),
            structure: py_config.structure,
            imports: py_config.imports,
            exports: py_config.exports,
            comments: py_config.comments,
            docstrings: py_config.docstrings,
            symbols: py_config.symbols,
            diagnostics: py_config.diagnostics,
            chunk_max_size: py_config.chunk_max_size,
        }
    }
}

// ---------------------------------------------------------------------------
// Process function
// ---------------------------------------------------------------------------

/// Convert a serde_json::Value to a Python object (dict, list, str, int, float, bool, None).
fn json_value_to_py(py: Python<'_>, value: &serde_json::Value) -> PyResult<Py<PyAny>> {
    match value {
        serde_json::Value::Null => Ok(py.None()),
        serde_json::Value::Bool(b) => Ok((*b).into_pyobject(py)?.to_owned().into_any().unbind()),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_pyobject(py)?.into_any().unbind())
            } else if let Some(f) = n.as_f64() {
                Ok(f.into_pyobject(py)?.into_any().unbind())
            } else {
                Ok(py.None())
            }
        }
        serde_json::Value::String(s) => Ok(s.as_str().into_pyobject(py)?.into_any().unbind()),
        serde_json::Value::Array(arr) => {
            let items: Vec<Py<PyAny>> = arr.iter().map(|v| json_value_to_py(py, v)).collect::<PyResult<_>>()?;
            Ok(PyList::new(py, &items)?.into_any().unbind())
        }
        serde_json::Value::Object(map) => {
            let dict = PyDict::new(py);
            for (k, v) in map {
                dict.set_item(k, json_value_to_py(py, v)?)?;
            }
            Ok(dict.into_any().unbind())
        }
    }
}

/// Process source code using a ProcessConfig and return the result as a Python dict.
#[pyfunction]
fn process(py: Python<'_>, source: &str, config: &ProcessConfig) -> PyResult<Py<PyAny>> {
    let core_config: ts_pack_core::ProcessConfig = config.into();
    let result = ts_pack_core::process(source, &core_config).map_err(|e| ParseError::new_err(format!("{e}")))?;
    let value = serde_json::to_value(&result).map_err(|e| ParseError::new_err(format!("serialization failed: {e}")))?;
    json_value_to_py(py, &value)
}

// ---------------------------------------------------------------------------
// Module registration
// ---------------------------------------------------------------------------

#[pymodule]
fn _native(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("LanguageNotFoundError", py.get_type::<LanguageNotFoundError>())?;
    m.add("ParseError", py.get_type::<ParseError>())?;
    m.add("QueryError", py.get_type::<QueryError>())?;
    m.add_class::<TreeHandle>()?;
    m.add_class::<ProcessConfig>()?;
    m.add_function(wrap_pyfunction!(get_binding, m)?)?;
    m.add_function(wrap_pyfunction!(get_language, m)?)?;
    m.add_function(wrap_pyfunction!(get_parser, m)?)?;
    m.add_function(wrap_pyfunction!(available_languages, m)?)?;
    m.add_function(wrap_pyfunction!(has_language, m)?)?;
    m.add_function(wrap_pyfunction!(language_count, m)?)?;
    m.add_function(wrap_pyfunction!(parse_string, m)?)?;
    m.add_function(wrap_pyfunction!(process, m)?)?;
    Ok(())
}
