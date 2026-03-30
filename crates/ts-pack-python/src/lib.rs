use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::path::Path;
use std::sync::Mutex;

/// Execute a closure with the Python GIL released.
fn without_gil<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    // SAFETY: GIL is held on entry (we're in a #[pyfunction]).
    // We release it so blocking Rust I/O doesn't stall Python threads,
    // then reacquire before returning to Python.
    // catch_unwind ensures GIL is restored even if f() panics.
    unsafe {
        let state = pyo3::ffi::PyEval_SaveThread();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
        pyo3::ffi::PyEval_RestoreThread(state);
        match result {
            Ok(v) => v,
            Err(e) => std::panic::resume_unwind(e),
        }
    }
}

pyo3::create_exception!(
    tree_sitter_language_pack,
    LanguageNotFoundError,
    pyo3::exceptions::PyValueError
);

pyo3::create_exception!(tree_sitter_language_pack, ParseError, pyo3::exceptions::PyRuntimeError);

pyo3::create_exception!(tree_sitter_language_pack, QueryError, pyo3::exceptions::PyValueError);

pyo3::create_exception!(
    tree_sitter_language_pack,
    DownloadError,
    pyo3::exceptions::PyRuntimeError
);

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
    let language =
        tree_sitter_language_pack::get_language(name).map_err(|e| LanguageNotFoundError::new_err(format!("{e}")))?;

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
    let langs = tree_sitter_language_pack::available_languages();
    let py_list = PyList::new(py, &langs)?;
    Ok(py_list.into_any().unbind())
}

/// Checks if a language is available.
#[pyfunction]
fn has_language(name: &str) -> bool {
    tree_sitter_language_pack::has_language(name)
}

/// Detect language name from a file path or extension.
///
/// Returns None if the extension is not recognized.
#[pyfunction]
fn detect_language(path: &str) -> Option<String> {
    tree_sitter_language_pack::detect_language_from_path(path).map(String::from)
}

/// Returns the number of available languages.
#[pyfunction]
fn language_count() -> usize {
    tree_sitter_language_pack::language_count()
}

/// Detect language name from file content (shebang-based detection).
///
/// Returns None if the content does not contain a recognized shebang.
#[pyfunction]
fn detect_language_from_content(content: &str) -> Option<String> {
    tree_sitter_language_pack::detect_language_from_content(content).map(String::from)
}

/// Detect language name from a bare file extension (without leading dot).
///
/// Returns None if the extension is not recognized.
#[pyfunction]
fn detect_language_from_extension(ext: &str) -> Option<String> {
    tree_sitter_language_pack::detect_language_from_extension(ext).map(String::from)
}

/// Detect language name from a file path based on its extension.
///
/// Returns None if the extension is not recognized.
#[pyfunction]
fn detect_language_from_path(path: &str) -> Option<String> {
    tree_sitter_language_pack::detect_language_from_path(path).map(String::from)
}

/// Index a workspace (Rust-native indexer). Returns list of indexed file paths.
#[pyfunction]
fn index_workspace(
    path: &str,
    project_id: &str,
    neo4j_uri: &str,
    neo4j_user: &str,
    neo4j_pass: &str,
    manifest_file: &str,
) -> PyResult<Vec<String>> {
    let config = ts_pack_index::IndexerConfig {
        neo4j_uri: neo4j_uri.to_string(),
        neo4j_user: neo4j_user.to_string(),
        neo4j_pass: neo4j_pass.to_string(),
        project_id: project_id.to_string(),
        manifest_file: Some(manifest_file.to_string()),
    };

    let result = without_gil(|| {
        let rt =
            tokio::runtime::Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(ts_pack_index::index_workspace(Path::new(path), config))
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    })?;

    Ok(result.into_iter().map(|p| p.to_string_lossy().into_owned()).collect())
}

/// Returns extension ambiguity information for the given file extension.
///
/// Returns a tuple of (assigned_language, alternative_languages) if the extension
/// is ambiguous, or None if the extension is not ambiguous.
#[pyfunction]
fn extension_ambiguity(ext: &str) -> Option<(String, Vec<String>)> {
    tree_sitter_language_pack::extension_ambiguity(ext)
        .map(|(assigned, alts)| (assigned.to_string(), alts.iter().map(|s| s.to_string()).collect()))
}

/// Returns the bundled highlights query for the given language, or None.
#[pyfunction]
fn get_highlights_query(language: &str) -> Option<String> {
    tree_sitter_language_pack::get_highlights_query(language).map(String::from)
}

/// Returns the bundled injections query for the given language, or None.
#[pyfunction]
fn get_injections_query(language: &str) -> Option<String> {
    tree_sitter_language_pack::get_injections_query(language).map(String::from)
}

/// Returns the bundled locals query for the given language, or None.
#[pyfunction]
fn get_locals_query(language: &str) -> Option<String> {
    tree_sitter_language_pack::get_locals_query(language).map(String::from)
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

impl TreeHandle {
    /// Acquire the tree lock and apply a closure, converting poison errors.
    fn with_tree<F, R>(&self, f: F) -> PyResult<R>
    where
        F: FnOnce(&tree_sitter::Tree) -> R,
    {
        let guard = self.inner.lock().map_err(|_| ParseError::new_err("lock poisoned"))?;
        Ok(f(&guard))
    }

    /// Acquire the tree lock and apply a fallible closure, converting poison errors.
    fn try_with_tree<F, R>(&self, f: F) -> PyResult<R>
    where
        F: FnOnce(&tree_sitter::Tree) -> PyResult<R>,
    {
        let guard = self.inner.lock().map_err(|_| ParseError::new_err("lock poisoned"))?;
        f(&guard)
    }
}

#[pymethods]
impl TreeHandle {
    /// Returns the type name of the root node.
    fn root_node_type(&self) -> PyResult<String> {
        self.with_tree(|tree| tree.root_node().kind().to_string())
    }

    /// Returns the number of named children of the root node.
    fn root_child_count(&self) -> PyResult<u32> {
        self.with_tree(|tree| tree.root_node().named_child_count() as u32)
    }

    /// Check whether any node in the tree has the given type name.
    fn contains_node_type(&self, node_type: &str) -> PyResult<bool> {
        self.with_tree(|tree| tree_sitter_language_pack::tree_contains_node_type(tree, node_type))
    }

    /// Check whether the tree contains any ERROR or MISSING nodes.
    fn has_error_nodes(&self) -> PyResult<bool> {
        self.with_tree(tree_sitter_language_pack::tree_has_error_nodes)
    }

    /// Returns the S-expression representation of the tree.
    fn to_sexp(&self) -> PyResult<String> {
        self.with_tree(tree_sitter_language_pack::tree_to_sexp)
    }

    /// Returns the count of ERROR and MISSING nodes in the tree.
    fn error_count(&self) -> PyResult<usize> {
        self.with_tree(tree_sitter_language_pack::tree_error_count)
    }

    /// Returns information about the root node as a dict.
    fn root_node_info(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        self.try_with_tree(|tree| {
            let info = tree_sitter_language_pack::root_node_info(tree);
            node_info_to_dict(py, &info)
        })
    }

    /// Finds all nodes matching the given type and returns their info as a list of dicts.
    fn find_nodes_by_type(&self, py: Python<'_>, node_type: &str) -> PyResult<Py<PyAny>> {
        self.try_with_tree(|tree| {
            let nodes = tree_sitter_language_pack::find_nodes_by_type(tree, node_type);
            let py_list: Vec<Py<PyAny>> = nodes
                .iter()
                .map(|info| node_info_to_dict(py, info))
                .collect::<PyResult<_>>()?;
            let list = PyList::new(py, &py_list)?;
            Ok(list.into_any().unbind())
        })
    }

    /// Returns info for all named children of the root node.
    fn named_children_info(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        self.try_with_tree(|tree| {
            let nodes = tree_sitter_language_pack::named_children_info(tree);
            let py_list: Vec<Py<PyAny>> = nodes
                .iter()
                .map(|info| node_info_to_dict(py, info))
                .collect::<PyResult<_>>()?;
            let list = PyList::new(py, &py_list)?;
            Ok(list.into_any().unbind())
        })
    }

    /// Extracts source text for a node given its start_byte and end_byte.
    fn extract_text(&self, start_byte: usize, end_byte: usize) -> PyResult<String> {
        let info = tree_sitter_language_pack::NodeInfo {
            kind: std::borrow::Cow::Borrowed(""),
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
        tree_sitter_language_pack::extract_text(&self.source, &info)
            .map(|s| s.to_string())
            .map_err(|e| ParseError::new_err(format!("{e}")))
    }

    /// Runs a tree-sitter query and returns matches as a list of dicts.
    fn run_query(&self, py: Python<'_>, language: &str, query_source: &str) -> PyResult<Py<PyAny>> {
        self.try_with_tree(|tree| {
            let matches = tree_sitter_language_pack::run_query(tree, language, query_source, &self.source)
                .map_err(|e| QueryError::new_err(format!("{e}")))?;

            let py_matches: Vec<Py<PyAny>> = matches
                .iter()
                .map(|m| query_match_to_dict(py, m))
                .collect::<PyResult<_>>()?;
            let list = PyList::new(py, &py_matches)?;
            Ok(list.into_any().unbind())
        })
    }
}

// ---------------------------------------------------------------------------
// Parsing
// ---------------------------------------------------------------------------

/// Parse source code with the named language, returning a TreeHandle.
#[pyfunction]
fn parse_string(language: &str, source: &str) -> PyResult<TreeHandle> {
    let source_bytes = source.as_bytes();
    let tree = tree_sitter_language_pack::parse_string(language, source_bytes)
        .map_err(|e| ParseError::new_err(format!("{e}")))?;
    Ok(TreeHandle {
        inner: Mutex::new(tree),
        source: source_bytes.to_vec(),
    })
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn node_info_to_dict(py: Python<'_>, info: &tree_sitter_language_pack::NodeInfo) -> PyResult<Py<PyAny>> {
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

fn query_match_to_dict(py: Python<'_>, qm: &tree_sitter_language_pack::QueryMatch) -> PyResult<Py<PyAny>> {
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

impl From<&ProcessConfig> for tree_sitter_language_pack::ProcessConfig {
    fn from(py_config: &ProcessConfig) -> Self {
        Self {
            language: std::borrow::Cow::Owned(py_config.language.clone()),
            structure: py_config.structure,
            imports: py_config.imports,
            exports: py_config.exports,
            comments: py_config.comments,
            docstrings: py_config.docstrings,
            symbols: py_config.symbols,
            diagnostics: py_config.diagnostics,
            chunk_max_size: py_config.chunk_max_size,
            extractions: None,
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
            } else if let Some(u) = n.as_u64() {
                Ok(u.into_pyobject(py)?.into_any().unbind())
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
    let core_config: tree_sitter_language_pack::ProcessConfig = config.into();
    let result =
        tree_sitter_language_pack::process(source, &core_config).map_err(|e| ParseError::new_err(format!("{e}")))?;
    let value = serde_json::to_value(&result).map_err(|e| ParseError::new_err(format!("serialization failed: {e}")))?;
    json_value_to_py(py, &value)
}

// ---------------------------------------------------------------------------
// Extraction API
// ---------------------------------------------------------------------------

/// Extract patterns from source code using an ExtractionConfig dict.
///
/// The config dict should have keys: "language" (str), "patterns" (dict of pattern dicts).
/// Each pattern dict has: "query" (str), optional "capture_output" (str),
/// optional "child_fields" (list[str]), optional "max_results" (int),
/// optional "byte_range" (tuple[int, int]).
///
/// Returns the extraction result as a Python dict.
#[pyfunction]
fn extract(py: Python<'_>, source: &str, config: &pyo3::Bound<'_, PyDict>) -> PyResult<Py<PyAny>> {
    let config_json = py_dict_to_json_value(config)?;
    let extraction_config: tree_sitter_language_pack::ExtractionConfig =
        serde_json::from_value(config_json).map_err(|e| ParseError::new_err(format!("invalid config: {e}")))?;
    let result = tree_sitter_language_pack::extract_patterns(source, &extraction_config)
        .map_err(|e| ParseError::new_err(format!("{e}")))?;
    let value = serde_json::to_value(&result).map_err(|e| ParseError::new_err(format!("serialization failed: {e}")))?;
    json_value_to_py(py, &value)
}

/// Validate extraction patterns without running them.
///
/// The config dict has the same shape as for `extract`.
/// Returns a dict with "valid" (bool) and "patterns" (dict of validation details).
#[pyfunction]
fn validate_extraction(py: Python<'_>, config: &pyo3::Bound<'_, PyDict>) -> PyResult<Py<PyAny>> {
    let config_json = py_dict_to_json_value(config)?;
    let extraction_config: tree_sitter_language_pack::ExtractionConfig =
        serde_json::from_value(config_json).map_err(|e| ParseError::new_err(format!("invalid config: {e}")))?;
    let result = tree_sitter_language_pack::validate_extraction(&extraction_config)
        .map_err(|e| ParseError::new_err(format!("{e}")))?;
    let value = serde_json::to_value(&result).map_err(|e| ParseError::new_err(format!("serialization failed: {e}")))?;
    json_value_to_py(py, &value)
}

/// Convert a Python dict to a serde_json::Value.
fn py_dict_to_json_value(dict: &pyo3::Bound<'_, PyDict>) -> PyResult<serde_json::Value> {
    let py = dict.py();
    let json_mod = py.import("json")?;
    let json_str: String = json_mod.call_method1("dumps", (dict,))?.extract()?;
    serde_json::from_str(&json_str).map_err(|e| ParseError::new_err(format!("JSON conversion failed: {e}")))
}

// ---------------------------------------------------------------------------
// Download API
// ---------------------------------------------------------------------------

/// Helper to convert a Python dict to a PackConfig.
fn dict_to_pack_config(dict: &pyo3::Bound<'_, PyDict>) -> PyResult<tree_sitter_language_pack::PackConfig> {
    let mut config = tree_sitter_language_pack::PackConfig::default();

    // Parse cache_dir if present
    if let Ok(Some(cache_dir_obj)) = dict.get_item("cache_dir")
        && !cache_dir_obj.is_none()
    {
        let path_str: String = cache_dir_obj.extract()?;
        config.cache_dir = Some(std::path::PathBuf::from(path_str));
    }

    // Parse languages if present
    if let Ok(Some(languages_obj)) = dict.get_item("languages")
        && !languages_obj.is_none()
    {
        let langs: Vec<String> = languages_obj.extract()?;
        config.languages = Some(langs);
    }

    // Parse groups if present
    if let Ok(Some(groups_obj)) = dict.get_item("groups")
        && !groups_obj.is_none()
    {
        let grps: Vec<String> = groups_obj.extract()?;
        config.groups = Some(grps);
    }

    Ok(config)
}

/// Initialize the language pack with configuration.
///
/// Applies cache directory settings and downloads languages/groups specified.
/// Accepts a dict with optional keys: cache_dir (str), languages (list[str]), groups (list[str]).
#[pyfunction]
fn init(_py: Python<'_>, config: &pyo3::Bound<'_, PyDict>) -> PyResult<()> {
    let pack_config = dict_to_pack_config(config)?;
    let result = without_gil(|| tree_sitter_language_pack::init(&pack_config));
    result.map_err(|e| DownloadError::new_err(e.to_string()))
}

/// Configure the language pack without downloading.
///
/// Sets a custom cache directory (or resets to default if None).
/// Keyword-only argument: cache_dir (str | None).
#[pyfunction]
#[pyo3(signature = (*, cache_dir = None))]
fn configure(_py: Python<'_>, cache_dir: Option<String>) -> PyResult<()> {
    let config = tree_sitter_language_pack::PackConfig {
        cache_dir: cache_dir.map(std::path::PathBuf::from),
        languages: None,
        groups: None,
    };
    tree_sitter_language_pack::configure(&config).map_err(|e| DownloadError::new_err(e.to_string()))
}

/// Download specific languages to the local cache.
///
/// Returns the number of newly downloaded languages.
#[pyfunction]
fn download(_py: Python<'_>, names: Vec<String>) -> PyResult<usize> {
    let refs: Vec<&str> = names.iter().map(String::as_str).collect();
    let result = without_gil(|| tree_sitter_language_pack::download(&refs));
    result.map_err(|e| DownloadError::new_err(e.to_string()))
}

/// Download all available languages from the remote manifest.
///
/// Returns the number of newly downloaded languages.
#[pyfunction]
fn download_all(_py: Python<'_>) -> PyResult<usize> {
    let result = without_gil(tree_sitter_language_pack::download_all);
    result.map_err(|e| DownloadError::new_err(e.to_string()))
}

/// Fetch all language names available in the remote manifest.
///
/// Returns a sorted list of all 248 downloadable languages.
#[pyfunction]
fn manifest_languages(py: Python<'_>) -> PyResult<Py<PyAny>> {
    let langs = without_gil(tree_sitter_language_pack::manifest_languages)
        .map_err(|e| DownloadError::new_err(e.to_string()))?;
    let py_list = PyList::new(py, &langs)?;
    Ok(py_list.into_any().unbind())
}

/// List languages that are already downloaded and cached locally.
///
/// Does not perform any network requests.
#[pyfunction]
fn downloaded_languages(py: Python<'_>) -> PyResult<Py<PyAny>> {
    let langs = tree_sitter_language_pack::downloaded_languages();
    let py_list = PyList::new(py, &langs)?;
    Ok(py_list.into_any().unbind())
}

/// Delete all cached parser shared libraries.
#[pyfunction]
fn clean_cache(_py: Python<'_>) -> PyResult<()> {
    let result = without_gil(tree_sitter_language_pack::clean_cache);
    result.map_err(|e| DownloadError::new_err(e.to_string()))
}

/// Return the effective cache directory path.
///
/// Returns either the custom path set via configure() or the default.
#[pyfunction]
fn cache_dir(_py: Python<'_>) -> PyResult<String> {
    let dir = tree_sitter_language_pack::cache_dir().map_err(|e| DownloadError::new_err(e.to_string()))?;
    Ok(dir.to_string_lossy().into_owned())
}

// ---------------------------------------------------------------------------
// Module registration
// ---------------------------------------------------------------------------

#[pymodule]
fn _native(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("LanguageNotFoundError", py.get_type::<LanguageNotFoundError>())?;
    m.add("ParseError", py.get_type::<ParseError>())?;
    m.add("QueryError", py.get_type::<QueryError>())?;
    m.add("DownloadError", py.get_type::<DownloadError>())?;
    m.add_class::<TreeHandle>()?;
    m.add_class::<ProcessConfig>()?;
    m.add_function(wrap_pyfunction!(get_binding, m)?)?;
    m.add_function(wrap_pyfunction!(get_language, m)?)?;
    m.add_function(wrap_pyfunction!(get_parser, m)?)?;
    m.add_function(wrap_pyfunction!(available_languages, m)?)?;
    m.add_function(wrap_pyfunction!(has_language, m)?)?;
    m.add_function(wrap_pyfunction!(detect_language, m)?)?;
    m.add_function(wrap_pyfunction!(language_count, m)?)?;
    m.add_function(wrap_pyfunction!(detect_language_from_content, m)?)?;
    m.add_function(wrap_pyfunction!(detect_language_from_extension, m)?)?;
    m.add_function(wrap_pyfunction!(detect_language_from_path, m)?)?;
    m.add_function(wrap_pyfunction!(extension_ambiguity, m)?)?;
    m.add_function(wrap_pyfunction!(index_workspace, m)?)?;
    m.add_function(wrap_pyfunction!(get_highlights_query, m)?)?;
    m.add_function(wrap_pyfunction!(get_injections_query, m)?)?;
    m.add_function(wrap_pyfunction!(get_locals_query, m)?)?;
    m.add_function(wrap_pyfunction!(parse_string, m)?)?;
    m.add_function(wrap_pyfunction!(process, m)?)?;
    m.add_function(wrap_pyfunction!(extract, m)?)?;
    m.add_function(wrap_pyfunction!(validate_extraction, m)?)?;
    m.add_function(wrap_pyfunction!(init, m)?)?;
    m.add_function(wrap_pyfunction!(configure, m)?)?;
    m.add_function(wrap_pyfunction!(download, m)?)?;
    m.add_function(wrap_pyfunction!(download_all, m)?)?;
    m.add_function(wrap_pyfunction!(manifest_languages, m)?)?;
    m.add_function(wrap_pyfunction!(downloaded_languages, m)?)?;
    m.add_function(wrap_pyfunction!(clean_cache, m)?)?;
    m.add_function(wrap_pyfunction!(cache_dir, m)?)?;
    Ok(())
}
