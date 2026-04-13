use futures::future::try_join_all;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PySet, PyTuple};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::path::Path;
use std::sync::Mutex;

mod graph_finalize;
mod swift_semantic;

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
#[pyo3(signature = (path, project_id, neo4j_uri, neo4j_user, neo4j_pass, manifest_file, status_project_id = None, run_id = None))]
fn index_workspace(
    path: &str,
    project_id: &str,
    neo4j_uri: &str,
    neo4j_user: &str,
    neo4j_pass: &str,
    manifest_file: &str,
    status_project_id: Option<String>,
    run_id: Option<String>,
) -> PyResult<Vec<String>> {
    let config = ts_pack_index::IndexerConfig {
        neo4j_uri: neo4j_uri.to_string(),
        neo4j_user: neo4j_user.to_string(),
        neo4j_pass: neo4j_pass.to_string(),
        project_id: project_id.to_string(),
        status_project_id,
        run_id,
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

#[pyfunction]
fn collapse_near_duplicate_texts(texts: Vec<String>) -> PyResult<Vec<usize>> {
    let indices = without_gil(|| {
        ts_pack_index::duplicate::select_non_duplicate_indices(
            &texts,
            ts_pack_index::duplicate::DuplicateCollapseConfig::for_search_results(),
        )
    });
    Ok(indices)
}

#[pyfunction]
fn analyze_duplicate_texts(
    py: Python<'_>,
    texts: Vec<String>,
    _query: Option<String>,
    mode: Option<String>,
    contexts_json: Option<String>,
) -> PyResult<Py<PyAny>> {
    let contexts: Vec<ts_pack_index::duplicate::CandidateContext> = contexts_json
        .as_deref()
        .map(|raw| serde_json::from_str(raw).unwrap_or_default())
        .unwrap_or_default();
    let result = without_gil(|| {
        ts_pack_index::duplicate::analyze_duplicates(
            &texts,
            ts_pack_index::duplicate::DuplicateCollapseConfig::for_search_results(),
            if mode.as_deref() == Some("docs") {
                "docs_retrieval"
            } else {
                "code_retrieval"
            },
            &contexts,
        )
    });
    let value = serde_json::to_value(&result).map_err(|e| ParseError::new_err(format!("serialization failed: {e}")))?;
    json_value_to_py(py, &value)
}

#[pyfunction]
fn rerank_diverse_texts(
    py: Python<'_>,
    texts: Vec<String>,
    relevance_scores: Vec<f64>,
    query: Option<String>,
    mode: Option<String>,
    contexts_json: Option<String>,
) -> PyResult<Py<PyAny>> {
    let contexts: Vec<ts_pack_index::duplicate::CandidateContext> = contexts_json
        .as_deref()
        .map(|raw| serde_json::from_str(raw).unwrap_or_default())
        .unwrap_or_default();
    let result = without_gil(|| {
        ts_pack_index::duplicate::rerank_diverse_for_search(
            &texts,
            &relevance_scores,
            query.as_deref(),
            mode.as_deref(),
            &contexts,
        )
    });
    let value = serde_json::to_value(&result).map_err(|e| ParseError::new_err(format!("serialization failed: {e}")))?;
    json_value_to_py(py, &value)
}

#[pyfunction]
fn trace_diverse_texts(
    py: Python<'_>,
    texts: Vec<String>,
    relevance_scores: Vec<f64>,
    query: Option<String>,
    mode: Option<String>,
    contexts_json: Option<String>,
    experiments_json: Option<String>,
) -> PyResult<Py<PyAny>> {
    let contexts: Vec<ts_pack_index::duplicate::CandidateContext> = contexts_json
        .as_deref()
        .map(|raw| serde_json::from_str(raw).unwrap_or_default())
        .unwrap_or_default();
    let experiments = parse_experiment_config(experiments_json.as_deref())?;
    let result = without_gil(|| {
        ts_pack_index::duplicate::rerank_diverse_trace_for_search_with_experiments(
            &texts,
            &relevance_scores,
            query.as_deref(),
            mode.as_deref(),
            &contexts,
            &experiments,
        )
    });
    let value = serde_json::to_value(&result).map_err(|e| ParseError::new_err(format!("serialization failed: {e}")))?;
    json_value_to_py(py, &value)
}

fn parse_experiment_config(raw: Option<&str>) -> PyResult<ts_pack_index::duplicate::ExperimentConfig> {
    let mut config = ts_pack_index::duplicate::ExperimentConfig::default();
    let Some(raw) = raw else {
        return Ok(config);
    };
    let value: serde_json::Value =
        serde_json::from_str(raw).map_err(|e| ParseError::new_err(format!("invalid experiments json: {e}")))?;
    let Some(obj) = value.as_object() else {
        return Ok(config);
    };

    let get_bool = |key: &str| obj.get(key).and_then(|v| v.as_bool());
    let get_f64 = |key: &str| obj.get(key).and_then(|v| v.as_f64());

    config.boilerplate_variant_suppression = get_bool("boilerplate_variant_suppression").unwrap_or(false);
    config.canonical_docs_mirror_suppression = get_bool("canonical_docs_mirror_suppression").unwrap_or(false);
    config.helper_clone_suppression = get_bool("helper_clone_suppression").unwrap_or(false);
    config.threshold_struct = get_f64("threshold_struct");
    config.threshold_lexical = get_f64("threshold_lexical");
    config.threshold_role = get_f64("threshold_role");
    config.min_length_ratio = get_f64("min_length_ratio");
    config.max_length_ratio = get_f64("max_length_ratio");
    config.threshold_query_distinction = get_f64("threshold_query_distinction");
    config.allow_cross_role_suppression = get_bool("allow_cross_role_suppression").unwrap_or(false);
    Ok(config)
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

/// Extract typed file facts such as route defs, HTTP calls, and resource refs.
#[pyfunction]
#[pyo3(signature = (source, language, file_path = None))]
fn extract_file_facts(py: Python<'_>, source: &str, language: &str, file_path: Option<String>) -> PyResult<Py<PyAny>> {
    let result = tree_sitter_language_pack::extract_file_facts(source, language, file_path.as_deref())
        .map_err(|e| ParseError::new_err(format!("{e}")))?;
    let value = serde_json::to_value(&result).map_err(|e| ParseError::new_err(format!("serialization failed: {e}")))?;
    json_value_to_py(py, &value)
}

/// Extract Swift semantic facts using SourceKitten/Xcode build context.
#[pyfunction]
fn extract_swift_semantic_facts(py: Python<'_>, project_path: &str) -> PyResult<Py<PyAny>> {
    let value = without_gil(|| swift_semantic::extract_swift_semantic_facts_value(project_path));
    json_value_to_py(py, &value)
}

/// Extract and persist Swift semantic graph enrichment using SourceKitten/Xcode context.
#[pyfunction]
#[pyo3(signature = (project_path, project_id, indexed_files, neo4j_uri, neo4j_user, neo4j_pass, neo4j_db = "proxy".to_string()))]
fn enrich_swift_graph(
    py: Python<'_>,
    project_path: &str,
    project_id: &str,
    indexed_files: Vec<String>,
    neo4j_uri: &str,
    neo4j_user: &str,
    neo4j_pass: &str,
    neo4j_db: String,
) -> PyResult<Py<PyAny>> {
    let value = without_gil(|| {
        let rt =
            tokio::runtime::Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(swift_semantic::enrich_swift_graph_async(
            project_path,
            project_id,
            &indexed_files,
            neo4j_uri,
            neo4j_user,
            neo4j_pass,
            &neo4j_db,
        ))
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    })?;
    json_value_to_py(py, &value)
}

/// Run the Rust-owned post-index graph finalization pipeline.
#[pyfunction]
#[pyo3(signature = (project_path, project_id, manifest_file, indexed_files, neo4j_uri, neo4j_user, neo4j_pass, neo4j_db = "proxy".to_string(), run_id = None))]
fn finalize_struct_graph(
    py: Python<'_>,
    project_path: &str,
    project_id: &str,
    manifest_file: &str,
    indexed_files: Vec<String>,
    neo4j_uri: &str,
    neo4j_user: &str,
    neo4j_pass: &str,
    neo4j_db: String,
    run_id: Option<String>,
) -> PyResult<Py<PyAny>> {
    let value = without_gil(|| {
        let rt =
            tokio::runtime::Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(graph_finalize::finalize_struct_graph_async(
            project_path,
            project_id,
            manifest_file,
            &indexed_files,
            neo4j_uri,
            neo4j_user,
            neo4j_pass,
            &neo4j_db,
            run_id.as_deref(),
        ))
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    })?;
    json_value_to_py(py, &value)
}

/// Prune stale structural graph data after a run has finalized successfully but
/// before it is promoted as the active shadow graph.
#[pyfunction]
#[pyo3(signature = (project_id, run_id, neo4j_uri, neo4j_user, neo4j_pass))]
fn prune_struct_shadow_graph(
    project_id: &str,
    run_id: &str,
    neo4j_uri: &str,
    neo4j_user: &str,
    neo4j_pass: &str,
) -> PyResult<()> {
    without_gil(|| {
        let rt =
            tokio::runtime::Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            let config = neo4rs::ConfigBuilder::default()
                .uri(neo4j_uri)
                .user(neo4j_user)
                .password(neo4j_pass)
                .fetch_size(1000)
                .max_connections(8)
                .build()
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            let graph = std::sync::Arc::new(
                neo4rs::Graph::connect(config)
                    .await
                    .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?,
            );
            ts_pack_index::prune_project_shadow_graph(&graph, project_id, run_id)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    })
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

#[pyfunction(signature = (all_chunks, existing_ids = None))]
fn build_semantic_sync_plan(
    py: Python<'_>,
    all_chunks: Vec<Vec<Py<PyAny>>>,
    existing_ids: Option<Vec<String>>,
) -> PyResult<Py<PyAny>> {
    let existing: HashSet<String> = existing_ids.unwrap_or_default().into_iter().collect();
    let new_chunks = PyList::empty(py);
    let prune_targets = PyList::empty(py);
    let mut total_chunks: usize = 0;

    for file_chunks in all_chunks {
        if file_chunks.is_empty() {
            continue;
        }
        total_chunks += file_chunks.len();

        let mut file_path: Option<String> = None;
        let mut chunk_ids: Vec<String> = Vec::new();

        for chunk in &file_chunks {
            let chunk_any = chunk.bind(py);
            let chunk_dict = chunk_any.cast::<PyDict>()?;
            if let Some(ref_id_any) = chunk_dict.get_item("ref_id")? {
                let ref_id = ref_id_any.extract::<String>()?;
                if !existing.contains(&ref_id) {
                    new_chunks.append(chunk_any)?;
                }
                chunk_ids.push(ref_id);
            }
            if file_path.is_none() {
                if let Some(meta_any) = chunk_dict.get_item("metadata")? {
                    if let Ok(meta_dict) = meta_any.cast::<PyDict>() {
                        if let Some(file_any) = meta_dict.get_item("file")? {
                            file_path = Some(file_any.extract::<String>()?);
                        }
                    }
                }
            }
        }

        if let Some(path) = file_path {
            if !chunk_ids.is_empty() {
                let target = PyDict::new(py);
                target.set_item("file_path", path)?;
                target.set_item("chunk_ids", chunk_ids)?;
                prune_targets.append(target)?;
            }
        }
    }

    let result = PyDict::new(py);
    result.set_item("new_chunks", &new_chunks)?;
    result.set_item("skipped_chunks", total_chunks.saturating_sub(new_chunks.len()))?;
    result.set_item("prune_targets", prune_targets)?;
    result.set_item("total_chunks", total_chunks)?;
    Ok(result.into_any().unbind())
}

#[pyfunction(signature = (batch, project_id, expected_dim = None, created_at = None))]
fn build_codebase_embedding_rows(
    py: Python<'_>,
    batch: Vec<Py<PyAny>>,
    project_id: String,
    expected_dim: Option<usize>,
    created_at: Option<f64>,
) -> PyResult<Py<PyAny>> {
    let rows = PyList::empty(py);
    let mut now = created_at;
    let json_mod = py.import("json")?;
    let dumps = json_mod.getattr("dumps")?;

    for item in batch {
        let item_any = item.bind(py);
        let item_dict = item_any.cast::<PyDict>()?;

        let Some(chunk_id_any) = item_dict.get_item("ref_id")? else {
            continue;
        };
        let chunk_id = chunk_id_any.extract::<String>()?;

        let Some(text_any) = item_dict.get_item("text")? else {
            continue;
        };
        let text = match text_any.extract::<String>() {
            Ok(v) => v,
            Err(_) => continue,
        };

        let Some(vector_any) = item_dict.get_item("vector")? else {
            continue;
        };
        let vector_list = match vector_any.cast::<PyList>() {
            Ok(v) => v,
            Err(_) => continue,
        };
        if let Some(dim) = expected_dim {
            if vector_list.len() != dim {
                continue;
            }
        }

        let created = if let Some(ts) = now {
            ts
        } else {
            let time_mod = py.import("time")?;
            let ts = time_mod.getattr("time")?.call0()?.extract::<f64>()?;
            now = Some(ts);
            ts
        };

        let metadata_obj = item_dict.get_item("metadata")?;
        let (file_path, chunk_index, metadata_json) = if let Some(meta_any) = metadata_obj {
            if let Ok(meta_dict) = meta_any.cast::<PyDict>() {
                let file_path = match meta_dict.get_item("file")? {
                    Some(v) => v.extract::<String>().unwrap_or_default(),
                    None => String::new(),
                };
                let chunk_index = match meta_dict.get_item("chunk_index")? {
                    Some(v) => v.extract::<i64>().ok(),
                    None => None,
                }
                .or_else(|| match meta_dict.get_item("start_line") {
                    Ok(Some(v)) => v.extract::<i64>().ok(),
                    _ => None,
                })
                .unwrap_or(0);
                let metadata_json = dumps.call1((meta_any,))?.extract::<String>()?;
                (file_path, chunk_index, metadata_json)
            } else {
                (String::new(), 0_i64, "{}".to_string())
            }
        } else {
            (String::new(), 0_i64, "{}".to_string())
        };

        let vector_text = {
            let mut values = Vec::with_capacity(vector_list.len());
            for value in vector_list.iter() {
                values.push(value.extract::<f64>()?.to_string());
            }
            format!("[{}]", values.join(","))
        };

        let ref_type = match item_dict.get_item("ref_type")? {
            Some(v) => v.extract::<String>().unwrap_or_else(|_| "code_chunk".to_string()),
            None => "code_chunk".to_string(),
        };

        let row = PyTuple::new(
            py,
            [
                chunk_id.into_pyobject(py)?.into_any(),
                project_id.clone().into_pyobject(py)?.into_any(),
                file_path.into_pyobject(py)?.into_any(),
                ref_type.into_pyobject(py)?.into_any(),
                chunk_index.into_pyobject(py)?.into_any(),
                text.into_pyobject(py)?.into_any(),
                vector_text.into_pyobject(py)?.into_any(),
                metadata_json.into_pyobject(py)?.into_any(),
                created.into_pyobject(py)?.into_any(),
            ],
        )?;
        rows.append(row)?;
    }

    Ok(rows.into_any().unbind())
}

fn chunk_id(project_id: &str, file_path: &str, start_byte: usize, text: &str, version: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("{file_path}:{start_byte}:{text}").as_bytes());
    let digest = hasher.finalize();
    let hex: String = digest.iter().map(|b| format!("{b:02x}")).collect();
    format!("{project_id}:{version}:{file_path}:{}", &hex[..14])
}

fn merge_metadata_dict<'py>(
    py: Python<'py>,
    metadata: &Bound<'py, PyDict>,
    file_meta: Option<&Bound<'py, PyAny>>,
) -> PyResult<()> {
    if let Some(extra) = file_meta {
        if let Ok(extra_dict) = extra.cast::<PyDict>() {
            for (k, v) in extra_dict.iter() {
                metadata.set_item(k, v)?;
            }
        }
    }
    let _ = py;
    Ok(())
}

fn value_as_str<'a>(value: &'a serde_json::Value) -> Option<&'a str> {
    value.as_str()
}

fn value_as_i64(value: &serde_json::Value) -> Option<i64> {
    value.as_i64().or_else(|| value.as_u64().map(|v| v as i64))
}

fn compact_imports(imports: &[serde_json::Value]) -> serde_json::Value {
    let mut out = Vec::new();
    for item in imports {
        let Some(obj) = item.as_object() else { continue };
        let source = obj
            .get("source")
            .and_then(value_as_str)
            .or_else(|| obj.get("module").and_then(value_as_str));
        let Some(source) = source else { continue };
        let names = obj
            .get("names")
            .and_then(|v| v.as_array())
            .map(|arr| serde_json::Value::Array(arr.iter().take(10).cloned().collect()))
            .unwrap_or_else(|| serde_json::Value::Array(Vec::new()));
        out.push(serde_json::json!({ "source": source, "names": names }));
        if out.len() >= 80 {
            break;
        }
    }
    serde_json::Value::Array(out)
}

fn compact_exports(exports: &[serde_json::Value]) -> serde_json::Value {
    let mut out = Vec::new();
    for item in exports {
        let Some(obj) = item.as_object() else { continue };
        let Some(name) = obj.get("name").and_then(value_as_str) else {
            continue;
        };
        out.push(serde_json::json!({
            "name": name,
            "kind": obj.get("kind").cloned().unwrap_or(serde_json::Value::Null),
        }));
        if out.len() >= 80 {
            break;
        }
    }
    serde_json::Value::Array(out)
}

fn compact_symbols(symbols: &[serde_json::Value]) -> serde_json::Value {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for item in symbols {
        let name = if let Some(obj) = item.as_object() {
            obj.get("name")
                .and_then(value_as_str)
                .or_else(|| obj.get("symbol").and_then(value_as_str))
                .or_else(|| obj.get("text").and_then(value_as_str))
        } else {
            item.as_str()
        };
        let Some(name) = name else { continue };
        if seen.insert(name.to_string()) {
            out.push(serde_json::Value::String(name.to_string()));
            if out.len() >= 200 {
                break;
            }
        }
    }
    serde_json::Value::Array(out)
}

fn compact_diagnostics(diags: &[serde_json::Value]) -> serde_json::Value {
    let items: Vec<serde_json::Value> = diags
        .iter()
        .take(10)
        .filter_map(|diag| {
            let obj = diag.as_object()?;
            let span = obj.get("span").and_then(|v| v.as_object());
            Some(serde_json::json!({
                "message": obj.get("message").cloned().unwrap_or(serde_json::Value::Null),
                "start_line": obj.get("start_line").cloned()
                    .or_else(|| span.and_then(|s| s.get("start_row").cloned()))
                    .unwrap_or(serde_json::Value::Null),
                "start_col": obj.get("start_col").cloned()
                    .or_else(|| span.and_then(|s| s.get("start_col").cloned()))
                    .unwrap_or(serde_json::Value::Null),
            }))
        })
        .collect();
    serde_json::json!({
        "count": diags.len(),
        "items": items,
    })
}

fn extract_metrics(metrics: Option<&serde_json::Map<String, serde_json::Value>>) -> serde_json::Value {
    let mut out = serde_json::Map::new();
    let Some(metrics) = metrics else {
        return serde_json::Value::Object(out);
    };
    for (out_key, keys) in [
        ("total_lines", ["total_lines", "totalLines"]),
        ("code_lines", ["code_lines", "codeLines"]),
        ("comment_lines", ["comment_lines", "commentLines"]),
        ("blank_lines", ["blank_lines", "blankLines"]),
        ("error_count", ["error_count", "errorCount"]),
        ("node_count", ["node_count", "nodeCount"]),
        ("max_depth", ["max_depth", "maxDepth"]),
        ("total_bytes", ["total_bytes", "totalBytes"]),
    ] {
        for key in keys {
            if let Some(value) = metrics.get(key) {
                out.insert(out_key.into(), value.clone());
                break;
            }
        }
    }
    serde_json::Value::Object(out)
}

fn compact_extractions(extractions: Option<&serde_json::Map<String, serde_json::Value>>) -> serde_json::Value {
    let mut out = serde_json::Map::new();
    let Some(extractions) = extractions else {
        return serde_json::Value::Object(out);
    };
    for (name, payload) in extractions {
        let Some(matches) = payload
            .as_object()
            .and_then(|p| p.get("matches"))
            .and_then(|v| v.as_array())
        else {
            continue;
        };
        let mut values = Vec::new();
        for item in matches.iter().take(50) {
            let Some(captures) = item
                .as_object()
                .and_then(|m| m.get("captures"))
                .and_then(|v| v.as_array())
            else {
                continue;
            };
            for capture in captures {
                if let Some(text) = capture.as_object().and_then(|c| c.get("text")).and_then(value_as_str) {
                    values.push(serde_json::Value::String(text.to_string()));
                }
            }
        }
        if !values.is_empty() {
            out.insert(
                name.clone(),
                serde_json::Value::Array(values.into_iter().take(50).collect()),
            );
        }
    }
    serde_json::Value::Object(out)
}

fn normalize_ts_pack_result(source: &str, language: &str, mut result: serde_json::Value) -> serde_json::Value {
    if !matches!(language, "typescript" | "tsx") {
        return result;
    }
    let Some(root) = result.as_object_mut() else {
        return result;
    };
    let Some(diags) = root.get("diagnostics").and_then(|v| v.as_array()) else {
        return result;
    };
    let lines: Vec<&str> = source.lines().collect();
    let mut filtered = Vec::new();
    let mut dropped = 0_i64;
    for diag in diags {
        let Some(obj) = diag.as_object() else {
            filtered.push(diag.clone());
            continue;
        };
        let message = obj.get("message").and_then(value_as_str).unwrap_or("");
        let line_idx = obj
            .get("span")
            .and_then(|v| v.as_object())
            .and_then(|span| span.get("start_line").or_else(|| span.get("start_row")))
            .and_then(value_as_i64)
            .unwrap_or(-1);
        let line_text = if line_idx >= 0 {
            lines.get(line_idx as usize).copied().unwrap_or("")
        } else {
            ""
        };
        let drop_diag =
            message == "Missing expected node: !" && line_text.contains("$queryRaw") && line_text.contains('`');
        if drop_diag {
            dropped += 1;
        } else {
            filtered.push(diag.clone());
        }
    }
    if dropped > 0 {
        root.insert("diagnostics".into(), serde_json::Value::Array(filtered));
        if let Some(metrics) = root.get_mut("metrics").and_then(|v| v.as_object_mut()) {
            for key in ["error_count", "errorCount"] {
                if let Some(value) = metrics.get_mut(key) {
                    if let Some(cur) = value_as_i64(value) {
                        *value = serde_json::Value::from((cur - dropped).max(0));
                    }
                }
            }
        }
    }
    result
}

#[pyfunction(signature = (source, language, file_path, project_id, chunk_id_version = "v6".to_string(), chunk_max_size = 4000, _chunk_overlap = 200))]
fn build_semantic_payload(
    py: Python<'_>,
    source: &str,
    language: String,
    file_path: String,
    project_id: String,
    chunk_id_version: String,
    chunk_max_size: usize,
    _chunk_overlap: usize,
) -> PyResult<Py<PyAny>> {
    let config = tree_sitter_language_pack::ProcessConfig {
        language: std::borrow::Cow::Owned(language.clone()),
        structure: true,
        imports: true,
        exports: true,
        comments: true,
        docstrings: true,
        symbols: true,
        diagnostics: true,
        chunk_max_size: if language == "swift" {
            None
        } else {
            Some(chunk_max_size)
        },
        extractions: None,
    };
    let raw_result =
        tree_sitter_language_pack::process(source, &config).map_err(|e| ParseError::new_err(format!("{e}")))?;
    let result = normalize_ts_pack_result(
        source,
        &language,
        serde_json::to_value(&raw_result).map_err(|e| ParseError::new_err(format!("serialization failed: {e}")))?,
    );
    let file_facts = tree_sitter_language_pack::extract_file_facts(source, &language, Some(file_path.as_str()))
        .map_err(|e| ParseError::new_err(format!("{e}")))?;
    let file_facts_value =
        serde_json::to_value(&file_facts).map_err(|e| ParseError::new_err(format!("serialization failed: {e}")))?;
    let result_obj = result
        .as_object()
        .ok_or_else(|| ParseError::new_err("process result was not an object"))?;
    let imports = result_obj
        .get("imports")
        .and_then(|v| v.as_array())
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let exports = result_obj
        .get("exports")
        .and_then(|v| v.as_array())
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let symbols = result_obj
        .get("symbols")
        .and_then(|v| v.as_array())
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let diagnostics = result_obj
        .get("diagnostics")
        .and_then(|v| v.as_array())
        .map(Vec::as_slice)
        .unwrap_or(&[]);

    let mut file_meta = serde_json::Map::new();
    file_meta.insert("file_imports".into(), compact_imports(imports));
    file_meta.insert("file_exports".into(), compact_exports(exports));
    file_meta.insert("file_symbols".into(), compact_symbols(symbols));
    file_meta.insert("file_diagnostics".into(), compact_diagnostics(diagnostics));
    file_meta.insert(
        "file_metrics".into(),
        extract_metrics(result_obj.get("metrics").and_then(|v| v.as_object())),
    );
    file_meta.insert(
        "file_extractions".into(),
        compact_extractions(result_obj.get("extractions").and_then(|v| v.as_object())),
    );
    if !file_facts_value.is_null()
        && !(file_facts_value.is_object() && file_facts_value.as_object().is_some_and(|m| m.is_empty()))
    {
        file_meta.insert("file_facts".into(), file_facts_value);
    }

    let file_header = format!("// File: {file_path}\n");
    let mut chunks = Vec::new();
    for chunk in result_obj
        .get("chunks")
        .and_then(|v| v.as_array())
        .into_iter()
        .flatten()
    {
        let Some(chunk_obj) = chunk.as_object() else { continue };
        let cmeta = chunk_obj.get("metadata").and_then(|v| v.as_object());
        if cmeta
            .and_then(|m| m.get("has_error_nodes"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            continue;
        }
        let content = chunk_obj.get("content").and_then(value_as_str).unwrap_or("");
        if content.trim().is_empty() {
            continue;
        }
        let start_byte = chunk_obj.get("start_byte").and_then(value_as_i64).unwrap_or(0).max(0) as usize;
        let mut meta = serde_json::Map::new();
        meta.insert("file".into(), serde_json::Value::String(file_path.clone()));
        meta.insert("project_id".into(), serde_json::Value::String(project_id.clone()));
        meta.insert("language".into(), serde_json::Value::String(language.clone()));
        meta.insert(
            "symbols".into(),
            cmeta
                .and_then(|m| m.get("symbols_defined"))
                .cloned()
                .unwrap_or_else(|| serde_json::Value::Array(Vec::new())),
        );
        meta.insert(
            "start_line".into(),
            serde_json::Value::from(chunk_obj.get("start_line").and_then(value_as_i64).unwrap_or(0) + 1),
        );
        meta.insert(
            "end_line".into(),
            serde_json::Value::from(chunk_obj.get("end_line").and_then(value_as_i64).unwrap_or(0) + 1),
        );
        for key in [
            "docstrings",
            "context_path",
            "node_types",
            "comments",
            "has_error_nodes",
        ] {
            meta.insert(
                key.into(),
                cmeta
                    .and_then(|m| m.get(key))
                    .cloned()
                    .unwrap_or(serde_json::Value::Null),
            );
        }
        for (key, value) in &file_meta {
            meta.insert(key.clone(), value.clone());
        }
        chunks.push(serde_json::json!({
            "ref_id": chunk_id(&project_id, &file_path, start_byte, content, &chunk_id_version),
            "text": format!("{file_header}{content}"),
            "metadata": meta,
        }));
    }

    json_value_to_py(
        py,
        &serde_json::json!({
            "result": result,
            "file_meta": file_meta,
            "chunks": chunks,
        }),
    )
}

#[pyfunction(signature = (source, file_path, project_id, language = None, file_meta = None, chunk_id_version = "v6", chunk_lines = 60, overlap_lines = 10))]
fn build_line_window_chunks(
    py: Python<'_>,
    source: &str,
    file_path: &str,
    project_id: &str,
    language: Option<String>,
    file_meta: Option<Py<PyAny>>,
    chunk_id_version: &str,
    chunk_lines: usize,
    overlap_lines: usize,
) -> PyResult<Py<PyAny>> {
    let safe_chunk_lines = chunk_lines.max(1);
    let step = safe_chunk_lines.saturating_sub(overlap_lines).max(1);
    let lines: Vec<&str> = source.split('\n').collect();
    let file_header = format!("// File: {file_path}\n");
    let chunks = PyList::empty(py);
    let extra_meta = file_meta.as_ref().map(|v| v.bind(py));

    let mut i = 0usize;
    while i < lines.len() {
        let end = std::cmp::min(i + safe_chunk_lines, lines.len());
        let block = &lines[i..end];
        if block.is_empty() {
            break;
        }
        let text = format!("{file_header}{}", block.join("\n"));
        let metadata = PyDict::new(py);
        metadata.set_item("file", file_path)?;
        metadata.set_item("project_id", project_id)?;
        metadata.set_item("language", language.clone())?;
        merge_metadata_dict(py, &metadata, extra_meta.as_ref().map(|b| b.as_any()))?;

        let chunk = PyDict::new(py);
        chunk.set_item("ref_id", chunk_id(project_id, file_path, i, &text, chunk_id_version))?;
        chunk.set_item("text", text)?;
        chunk.set_item("metadata", metadata)?;
        chunks.append(chunk)?;
        i = i.saturating_add(step);
    }

    Ok(chunks.into_any().unbind())
}

#[pyfunction(signature = (source, file_path, project_id, file_meta = None, chunk_id_version = "v6", chunk_max_size = 4000, chunk_lines = 60, overlap_lines = 10))]
fn build_swift_chunks(
    py: Python<'_>,
    source: &str,
    file_path: &str,
    project_id: &str,
    file_meta: Option<Py<PyAny>>,
    chunk_id_version: &str,
    chunk_max_size: usize,
    chunk_lines: usize,
    overlap_lines: usize,
) -> PyResult<Py<PyAny>> {
    let Ok(language) = tree_sitter_language_pack::get_language("swift") else {
        return Ok(PyList::empty(py).into_any().unbind());
    };
    let mut parser = tree_sitter::Parser::new();
    if parser.set_language(&language).is_err() {
        return Ok(PyList::empty(py).into_any().unbind());
    }

    let src_b = source.as_bytes();
    let Some(tree) = parser.parse(src_b, None) else {
        return Ok(PyList::empty(py).into_any().unbind());
    };

    let member_types: HashSet<&str> = [
        "property_declaration",
        "function_declaration",
        "subscript_declaration",
        "typealias_declaration",
        "init_declaration",
        "deinit_declaration",
        "protocol_function_declaration",
        "protocol_property_declaration",
        "enum_entry",
    ]
    .into_iter()
    .collect();
    let container_types: HashSet<&str> = [
        "class_declaration",
        "struct_declaration",
        "enum_declaration",
        "protocol_declaration",
        "extension_declaration",
    ]
    .into_iter()
    .collect();

    let file_header = format!("// File: {file_path}\n");
    let chunks = PyList::empty(py);
    let extra_meta = file_meta.as_ref().map(|v| v.bind(py));
    let safe_chunk_lines = chunk_lines.max(1);
    let step = safe_chunk_lines.saturating_sub(overlap_lines).max(1);

    fn node_text(src_b: &[u8], node: tree_sitter::Node<'_>) -> String {
        String::from_utf8_lossy(&src_b[node.start_byte()..node.end_byte()]).into_owned()
    }

    fn name_of(src_b: &[u8], node: tree_sitter::Node<'_>) -> String {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            let kind = child.kind();
            if kind == "pattern" || kind == "simple_identifier" || kind == "type_identifier" {
                return node_text(src_b, child);
            }
        }
        String::new()
    }

    struct EmitCtx<'py> {
        py: Python<'py>,
        file_path: &'py str,
        project_id: &'py str,
        chunk_id_version: &'py str,
        chunk_max_size: usize,
        chunk_lines: usize,
        step: usize,
        file_header: &'py str,
        chunks: &'py Bound<'py, PyList>,
        extra_meta: Option<Bound<'py, PyAny>>,
    }

    fn append_chunk(
        ctx: &EmitCtx<'_>,
        text: &str,
        start_byte: usize,
        name: &str,
        start_line: usize,
        end_line: usize,
        context_path: &[String],
    ) -> PyResult<()> {
        if text.trim().is_empty() {
            return Ok(());
        }
        let metadata = PyDict::new(ctx.py);
        metadata.set_item("file", ctx.file_path)?;
        metadata.set_item("project_id", ctx.project_id)?;
        metadata.set_item("language", "swift")?;
        metadata.set_item(
            "symbols",
            if name.is_empty() {
                vec![]
            } else {
                vec![name.to_string()]
            },
        )?;
        metadata.set_item("start_line", start_line)?;
        metadata.set_item("end_line", end_line)?;
        metadata.set_item("context_path", context_path.to_vec())?;
        merge_metadata_dict(ctx.py, &metadata, ctx.extra_meta.as_ref().map(|b| b.as_any()))?;

        let chunk = PyDict::new(ctx.py);
        chunk.set_item(
            "ref_id",
            chunk_id(ctx.project_id, ctx.file_path, start_byte, text, ctx.chunk_id_version),
        )?;
        chunk.set_item("text", format!("{}{}", ctx.file_header, text))?;
        chunk.set_item("metadata", metadata)?;
        ctx.chunks.append(chunk)?;
        Ok(())
    }

    fn emit_text(
        ctx: &EmitCtx<'_>,
        text: &str,
        start_byte: usize,
        name: &str,
        start_line: usize,
        end_line: usize,
        context_path: &[String],
    ) -> PyResult<()> {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            return Ok(());
        }
        if trimmed.len() <= ctx.chunk_max_size {
            return append_chunk(ctx, trimmed, start_byte, name, start_line, end_line, context_path);
        }
        let lines: Vec<&str> = trimmed.split('\n').collect();
        let mut i = 0usize;
        while i < lines.len() {
            let end = std::cmp::min(i + ctx.chunk_lines, lines.len());
            let block = lines[i..end].join("\n");
            if !block.trim().is_empty() {
                append_chunk(
                    ctx,
                    block.trim(),
                    start_byte + i,
                    name,
                    start_line + i,
                    start_line + end.saturating_sub(1),
                    context_path,
                )?;
            }
            i = i.saturating_add(ctx.step);
        }
        Ok(())
    }

    fn walk_swift(
        ctx: &EmitCtx<'_>,
        src_b: &[u8],
        node: tree_sitter::Node<'_>,
        member_types: &HashSet<&str>,
        container_types: &HashSet<&str>,
        context_path: &[String],
    ) -> PyResult<()> {
        if member_types.contains(node.kind()) {
            let name = name_of(src_b, node);
            let mut next_context = context_path.to_vec();
            if !name.is_empty() {
                next_context.push(name.clone());
            }
            let text = node_text(src_b, node);
            return emit_text(
                ctx,
                &text,
                node.start_byte(),
                &name,
                node.start_position().row + 1,
                node.end_position().row + 1,
                &next_context,
            );
        }

        let mut next_context = context_path.to_vec();
        if container_types.contains(node.kind()) {
            let name = name_of(src_b, node);
            if !name.is_empty() {
                next_context.push(name);
            }
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            walk_swift(ctx, src_b, child, member_types, container_types, &next_context)?;
        }
        Ok(())
    }

    {
        let ctx = EmitCtx {
            py,
            file_path,
            project_id,
            chunk_id_version,
            chunk_max_size,
            chunk_lines: safe_chunk_lines,
            step,
            file_header: &file_header,
            chunks: &chunks,
            extra_meta: extra_meta.cloned(),
        };

        walk_swift(&ctx, src_b, tree.root_node(), &member_types, &container_types, &[])?;
    }
    Ok(chunks.into_any().unbind())
}

#[pyfunction]
fn build_semantic_index_round_plan(
    py: Python<'_>,
    new_chunks: Vec<Py<PyAny>>,
    batch_size: usize,
    concurrency: usize,
) -> PyResult<Py<PyAny>> {
    let safe_batch_size = batch_size.max(1);
    let safe_concurrency = concurrency.max(1);
    let window = safe_batch_size.saturating_mul(safe_concurrency).max(1);
    let total_new = new_chunks.len();
    let rounds = if total_new == 0 { 0 } else { total_new.div_ceil(window) };

    let result = PyList::empty(py);
    for round_idx in 0..rounds {
        let start = round_idx * window;
        let end = std::cmp::min(start + window, total_new);
        let group = &new_chunks[start..end];
        let sub_batches = PyList::empty(py);
        for batch in group.chunks(safe_batch_size) {
            let sub_batch = PyList::empty(py);
            for item in batch {
                sub_batch.append(item.bind(py))?;
            }
            sub_batches.append(sub_batch)?;
        }

        let round_payload = PyDict::new(py);
        round_payload.set_item("round_index", round_idx)?;
        round_payload.set_item("rounds", rounds)?;
        round_payload.set_item("group_size", group.len())?;
        round_payload.set_item("batch_count", sub_batches.len())?;
        round_payload.set_item("sub_batches", sub_batches)?;
        result.append(round_payload)?;
    }

    Ok(result.into_any().unbind())
}

#[pyfunction(signature = (all_chunks, existing_ids = None, manifest_paths = None, db_paths = None, rebuild = false, batch_size = 128, concurrency = 4))]
fn build_semantic_index_driver_plan(
    py: Python<'_>,
    all_chunks: Vec<Vec<Py<PyAny>>>,
    existing_ids: Option<Vec<String>>,
    manifest_paths: Option<Vec<String>>,
    db_paths: Option<Vec<String>>,
    rebuild: bool,
    batch_size: usize,
    concurrency: usize,
) -> PyResult<Py<PyAny>> {
    let existing: HashSet<String> = existing_ids.unwrap_or_default().into_iter().collect();
    let manifest_path_set: HashSet<String> = manifest_paths.unwrap_or_default().into_iter().collect();
    let db_path_set: HashSet<String> = db_paths.unwrap_or_default().into_iter().collect();
    let orphan_paths: Vec<String> = db_path_set.difference(&manifest_path_set).cloned().collect();

    let new_chunks = PyList::empty(py);
    let prune_targets = PyList::empty(py);
    let mut total_chunks: usize = 0;

    for file_chunks in all_chunks {
        if file_chunks.is_empty() {
            continue;
        }
        total_chunks += file_chunks.len();

        let mut file_path: Option<String> = None;
        let mut chunk_ids: Vec<String> = Vec::new();

        for chunk in &file_chunks {
            let chunk_any = chunk.bind(py);
            let chunk_dict = chunk_any.cast::<PyDict>()?;
            if let Some(ref_id_any) = chunk_dict.get_item("ref_id")? {
                let ref_id = ref_id_any.extract::<String>()?;
                if !existing.contains(&ref_id) {
                    new_chunks.append(chunk_any)?;
                }
                chunk_ids.push(ref_id);
            }
            if file_path.is_none() {
                if let Some(meta_any) = chunk_dict.get_item("metadata")? {
                    if let Ok(meta_dict) = meta_any.cast::<PyDict>() {
                        if let Some(file_any) = meta_dict.get_item("file")? {
                            file_path = Some(file_any.extract::<String>()?);
                        }
                    }
                }
            }
        }

        if let Some(path) = file_path {
            if !chunk_ids.is_empty() {
                let target = PyDict::new(py);
                target.set_item("file_path", path)?;
                target.set_item("chunk_ids", chunk_ids)?;
                prune_targets.append(target)?;
            }
        }
    }

    let round_plan = build_semantic_index_round_plan(
        py,
        new_chunks.iter().map(|item| item.clone().unbind()).collect(),
        batch_size,
        concurrency,
    )?;

    let result = PyDict::new(py);
    result.set_item("new_chunks", &new_chunks)?;
    result.set_item("skipped_chunks", total_chunks.saturating_sub(new_chunks.len()))?;
    result.set_item("prune_targets", prune_targets)?;
    result.set_item("total_chunks", total_chunks)?;
    result.set_item("orphan_paths", orphan_paths)?;
    result.set_item("wiped", rebuild)?;
    result.set_item("round_plan", round_plan)?;
    Ok(result.into_any().unbind())
}

fn rowcount_from_cursor(_py: Python<'_>, cursor: &Bound<'_, PyAny>) -> PyResult<i64> {
    Ok(cursor
        .getattr("rowcount")
        .ok()
        .and_then(|value| value.extract::<i64>().ok())
        .unwrap_or(0))
}

fn extract_first_col_strings(rows: &Bound<'_, PyAny>) -> PyResult<Vec<String>> {
    let list = rows.cast::<PyList>()?;
    let mut out = Vec::with_capacity(list.len());
    for row in list.iter() {
        if let Ok(tuple) = row.cast::<PyTuple>() {
            if !tuple.is_empty() {
                out.push(tuple.get_item(0)?.extract::<String>()?);
            }
            continue;
        }
        if let Ok(value) = row.extract::<String>() {
            out.push(value);
        }
    }
    Ok(out)
}

async fn await_python_awaitable(awaitable: Py<PyAny>) -> PyResult<Py<PyAny>> {
    let fut = Python::attach(|py| pyo3_async_runtimes::tokio::into_future(awaitable.into_bound(py)))?;
    fut.await
}

fn clone_py(obj: &Py<PyAny>) -> Py<PyAny> {
    Python::attach(|py| obj.clone_ref(py))
}

async fn await_py_method1(obj: &Py<PyAny>, method: &str, args: Py<PyTuple>) -> PyResult<Py<PyAny>> {
    let awaitable = Python::attach(|py| -> PyResult<Py<PyAny>> {
        let result = obj.bind(py).call_method1(method, args.into_bound(py))?;
        Ok(result.unbind())
    })?;
    await_python_awaitable(awaitable).await
}

async fn await_py_callable1(callable: &Py<PyAny>, arg: Py<PyAny>) -> PyResult<Py<PyAny>> {
    let awaitable = Python::attach(|py| -> PyResult<Py<PyAny>> {
        let result = callable.bind(py).call1((arg,))?;
        Ok(result.unbind())
    })?;
    await_python_awaitable(awaitable).await
}

#[pyfunction(signature = (cursor, batch, project_id, expected_dim = None, created_at = None))]
fn execute_codebase_embedding_upsert(
    py: Python<'_>,
    cursor: Py<PyAny>,
    batch: Vec<Py<PyAny>>,
    project_id: String,
    expected_dim: Option<usize>,
    created_at: Option<f64>,
) -> PyResult<Py<PyAny>> {
    let cursor_obj = cursor.clone_ref(py);
    Ok(pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let rows = Python::attach(|py| build_codebase_embedding_rows(py, batch, project_id, expected_dim, created_at))?;

        let row_count = Python::attach(|py| -> PyResult<usize> { Ok(rows.bind(py).cast::<PyList>()?.len()) })?;
        if row_count == 0 {
            return Python::attach(|py| Ok(0_i64.into_pyobject(py)?.into_any().unbind()));
        }

        let sql = "INSERT INTO codebase_embeddings
  (chunk_id, project_id, file_path, ref_type, chunk_index,
   content, embedding, metadata, created_at)
VALUES (%s, %s, %s, %s, %s, %s, %s::vector, %s::jsonb, to_timestamp(%s))
ON CONFLICT (chunk_id) DO NOTHING";

        let args = Python::attach(|py| -> PyResult<Py<PyTuple>> {
            Ok(PyTuple::new(
                py,
                [
                    sql.into_pyobject(py)?.into_any(),
                    rows.bind(py).clone().into_any().unbind().into_bound(py).into_any(),
                ],
            )?
            .unbind())
        })?;
        let _ = await_py_method1(&cursor_obj, "executemany", args).await?;
        Python::attach(|py| Ok((row_count as i64).into_pyobject(py)?.into_any().unbind()))
    })?
    .unbind())
}

#[pyfunction(signature = (conn, project_id, manifest_paths, all_chunks, embed_batch_fn, write_batch_fn, rebuild = false, batch_size = 128, concurrency = 4, progress_fn = None))]
fn execute_semantic_index_driver(
    py: Python<'_>,
    conn: Py<PyAny>,
    project_id: String,
    manifest_paths: Vec<String>,
    all_chunks: Vec<Vec<Py<PyAny>>>,
    embed_batch_fn: Py<PyAny>,
    write_batch_fn: Py<PyAny>,
    rebuild: bool,
    batch_size: usize,
    concurrency: usize,
    progress_fn: Option<Py<PyAny>>,
) -> PyResult<Py<PyAny>> {
    let conn_obj = conn.clone_ref(py);
    let embed_fn_obj = embed_batch_fn.clone_ref(py);
    let write_fn_obj = write_batch_fn.clone_ref(py);
    let progress_fn_obj = progress_fn.map(|p| p.clone_ref(py));

    Ok(pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let existing_args = Python::attach(|py| -> PyResult<Py<PyTuple>> {
            Ok(PyTuple::new(
                py,
                [
                    "SELECT chunk_id FROM codebase_embeddings WHERE project_id = %s"
                        .into_pyobject(py)?
                        .into_any(),
                    vec![project_id.clone()].into_pyobject(py)?.into_any(),
                ],
            )?
            .unbind())
        })?;
        let existing_rows = await_py_method1(&conn_obj, "execute", existing_args).await?;
        let existing_fetch = await_py_method1(
            &existing_rows,
            "fetchall",
            Python::attach(|py| -> PyResult<Py<PyTuple>> { Ok(PyTuple::empty(py).unbind()) })?,
        )
        .await?;
        let existing_ids = Python::attach(|py| extract_first_col_strings(&existing_fetch.bind(py)))?;

        let db_paths_args = Python::attach(|py| -> PyResult<Py<PyTuple>> {
            Ok(PyTuple::new(
                py,
                [
                    "SELECT DISTINCT file_path FROM codebase_embeddings WHERE project_id = %s"
                        .into_pyobject(py)?
                        .into_any(),
                    vec![project_id.clone()].into_pyobject(py)?.into_any(),
                ],
            )?
            .unbind())
        })?;
        let db_path_rows = await_py_method1(&conn_obj, "execute", db_paths_args).await?;
        let db_path_fetch = await_py_method1(
            &db_path_rows,
            "fetchall",
            Python::attach(|py| -> PyResult<Py<PyTuple>> { Ok(PyTuple::empty(py).unbind()) })?,
        )
        .await?;
        let db_paths = Python::attach(|py| extract_first_col_strings(&db_path_fetch.bind(py)))?;

        let driver_plan = Python::attach(|py| -> PyResult<Py<PyAny>> {
            build_semantic_index_driver_plan(
                py,
                all_chunks,
                Some(existing_ids.clone()),
                Some(manifest_paths.clone()),
                Some(db_paths),
                rebuild,
                batch_size,
                concurrency,
            )
        })?;

        if rebuild {
            let wipe_args = Python::attach(|py| -> PyResult<Py<PyTuple>> {
                Ok(PyTuple::new(
                    py,
                    [
                        "DELETE FROM codebase_embeddings WHERE project_id = %s"
                            .into_pyobject(py)?
                            .into_any(),
                        vec![project_id.clone()].into_pyobject(py)?.into_any(),
                    ],
                )?
                .unbind())
            })?;
            let _ = await_py_method1(&conn_obj, "execute", wipe_args).await?;
        }

        let (orphan_paths, prune_targets, skipped_chunks, total_chunks, round_plan, new_chunks) =
            Python::attach(|py| -> PyResult<_> {
                let plan = driver_plan.bind(py).cast::<PyDict>()?;
                Ok((
                    plan.get_item("orphan_paths")?
                        .map(|v| v.extract::<Vec<String>>())
                        .transpose()?
                        .unwrap_or_default(),
                    plan.get_item("prune_targets")?
                        .map(|v| v.extract::<Vec<Py<PyAny>>>())
                        .transpose()?
                        .unwrap_or_default(),
                    plan.get_item("skipped_chunks")?
                        .map(|v| v.extract::<usize>())
                        .transpose()?
                        .unwrap_or(0),
                    plan.get_item("total_chunks")?
                        .map(|v| v.extract::<usize>())
                        .transpose()?
                        .unwrap_or(0),
                    plan.get_item("round_plan")?
                        .map(|v| v.extract::<Vec<Py<PyAny>>>())
                        .transpose()?
                        .unwrap_or_default(),
                    plan.get_item("new_chunks")?
                        .map(|v| v.extract::<Vec<Py<PyAny>>>())
                        .transpose()?
                        .unwrap_or_default(),
                ))
            })?;

        let mut orphan_pruned = 0_i64;
        for path in &orphan_paths {
            let args = Python::attach(|py| -> PyResult<Py<PyTuple>> {
                Ok(PyTuple::new(
                    py,
                    [
                        "DELETE FROM codebase_embeddings WHERE project_id = %s AND file_path = %s"
                            .into_pyobject(py)?
                            .into_any(),
                        (project_id.clone(), path.clone()).into_pyobject(py)?.into_any(),
                    ],
                )?
                .unbind())
            })?;
            let _ = await_py_method1(&conn_obj, "execute", args).await?;
            orphan_pruned += 1;
        }

        let mut pruned_total = 0_i64;
        for target in &prune_targets {
            let (file_path, chunk_ids) = Python::attach(|py| -> PyResult<(String, Vec<String>)> {
                let dict = target.bind(py).cast::<PyDict>()?;
                Ok((
                    dict.get_item("file_path")?
                        .ok_or_else(|| ParseError::new_err("missing file_path in prune target"))?
                        .extract::<String>()?,
                    dict.get_item("chunk_ids")?
                        .ok_or_else(|| ParseError::new_err("missing chunk_ids in prune target"))?
                        .extract::<Vec<String>>()?,
                ))
            })?;
            let args = Python::attach(|py| -> PyResult<Py<PyTuple>> {
                Ok(PyTuple::new(
                    py,
                    [
                        "
                        DELETE FROM codebase_embeddings
                        WHERE project_id = %s
                          AND file_path = %s
                          AND NOT (chunk_id = ANY(%s))
                    "
                        .into_pyobject(py)?
                        .into_any(),
                        (project_id.clone(), file_path, chunk_ids).into_pyobject(py)?.into_any(),
                    ],
                )?
                .unbind())
            })?;
            let cursor = await_py_method1(&conn_obj, "execute", args).await?;
            pruned_total += Python::attach(|py| rowcount_from_cursor(py, &cursor.bind(py)))?;
        }

        let total_new = new_chunks.len();
        let mut total_written = 0_i64;
        let rounds = round_plan.len();

        for round_info in &round_plan {
            let (round_index, round_count, group_size, batch_count, sub_batches) =
                Python::attach(|py| -> PyResult<(usize, usize, usize, usize, Vec<Py<PyAny>>)> {
                    let dict = round_info.bind(py).cast::<PyDict>()?;
                    Ok((
                        dict.get_item("round_index")?
                            .and_then(|v| v.extract::<usize>().ok())
                            .unwrap_or(0),
                        dict.get_item("rounds")?
                            .and_then(|v| v.extract::<usize>().ok())
                            .unwrap_or(rounds.max(1)),
                        dict.get_item("group_size")?
                            .and_then(|v| v.extract::<usize>().ok())
                            .unwrap_or(0),
                        dict.get_item("batch_count")?
                            .and_then(|v| v.extract::<usize>().ok())
                            .unwrap_or(0),
                        dict.get_item("sub_batches")?
                            .map(|v| v.extract::<Vec<Py<PyAny>>>())
                            .transpose()?
                            .unwrap_or_default(),
                    ))
                })?;

            if let Some(progress) = &progress_fn_obj {
                let payload = Python::attach(|py| -> PyResult<Py<PyAny>> {
                    let d = PyDict::new(py);
                    d.set_item("round_index", round_index)?;
                    d.set_item("rounds", round_count)?;
                    d.set_item("group_size", group_size)?;
                    d.set_item("batch_count", batch_count)?;
                    d.set_item("written_so_far", total_written)?;
                    d.set_item("total_new", total_new)?;
                    d.set_item("phase", "embed_start")?;
                    Ok(d.into_any().unbind())
                })?;
                let _ = await_py_callable1(progress, payload).await?;
            }

            let embedded_batches = try_join_all(
                sub_batches
                    .iter()
                    .map(|batch| await_py_callable1(&embed_fn_obj, clone_py(batch))),
            )
            .await?;

            if let Some(progress) = &progress_fn_obj {
                let payload = Python::attach(|py| -> PyResult<Py<PyAny>> {
                    let d = PyDict::new(py);
                    d.set_item("round_index", round_index)?;
                    d.set_item("rounds", round_count)?;
                    d.set_item("group_size", group_size)?;
                    d.set_item("batch_count", batch_count)?;
                    d.set_item("written_so_far", total_written)?;
                    d.set_item("total_new", total_new)?;
                    d.set_item("phase", "write_start")?;
                    Ok(d.into_any().unbind())
                })?;
                let _ = await_py_callable1(progress, payload).await?;
            }

            let write_counts = try_join_all(
                embedded_batches
                    .iter()
                    .map(|batch| await_py_callable1(&write_fn_obj, clone_py(batch))),
            )
            .await?;
            let round_written = Python::attach(|py| -> PyResult<i64> {
                let mut total = 0_i64;
                for count in &write_counts {
                    total += count.bind(py).extract::<i64>()?;
                }
                Ok(total)
            })?;
            total_written += round_written;

            if let Some(progress) = &progress_fn_obj {
                let payload = Python::attach(|py| -> PyResult<Py<PyAny>> {
                    let d = PyDict::new(py);
                    d.set_item("round_index", round_index)?;
                    d.set_item("rounds", round_count)?;
                    d.set_item("group_size", group_size)?;
                    d.set_item("batch_count", batch_count)?;
                    d.set_item("written_so_far", total_written)?;
                    d.set_item("total_new", total_new)?;
                    d.set_item("phase", "round_done")?;
                    d.set_item("round_written", round_written)?;
                    Ok(d.into_any().unbind())
                })?;
                let _ = await_py_callable1(progress, payload).await?;
            }
        }

        let result = Python::attach(|py| -> PyResult<Py<PyAny>> {
            let result = PyDict::new(py);
            result.set_item("new_chunks", new_chunks)?;
            result.set_item("skipped_chunks", skipped_chunks)?;
            result.set_item("prune_targets", prune_targets)?;
            result.set_item("total_chunks", total_chunks)?;
            result.set_item("existing_ids", PySet::new(py, existing_ids)?)?;
            result.set_item("wiped", rebuild)?;
            result.set_item("orphan_pruned", orphan_pruned)?;
            result.set_item("pruned_total", pruned_total)?;
            result.set_item("written", total_written)?;
            result.set_item("rounds", rounds)?;
            Ok(result.into_any().unbind())
        })?;
        Ok(result)
    })?
    .unbind())
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
    m.add_function(wrap_pyfunction!(collapse_near_duplicate_texts, m)?)?;
    m.add_function(wrap_pyfunction!(analyze_duplicate_texts, m)?)?;
    m.add_function(wrap_pyfunction!(rerank_diverse_texts, m)?)?;
    m.add_function(wrap_pyfunction!(trace_diverse_texts, m)?)?;
    m.add_function(wrap_pyfunction!(get_highlights_query, m)?)?;
    m.add_function(wrap_pyfunction!(get_injections_query, m)?)?;
    m.add_function(wrap_pyfunction!(get_locals_query, m)?)?;
    m.add_function(wrap_pyfunction!(parse_string, m)?)?;
    m.add_function(wrap_pyfunction!(process, m)?)?;
    m.add_function(wrap_pyfunction!(extract, m)?)?;
    m.add_function(wrap_pyfunction!(validate_extraction, m)?)?;
    m.add_function(wrap_pyfunction!(extract_file_facts, m)?)?;
    m.add_function(wrap_pyfunction!(extract_swift_semantic_facts, m)?)?;
    m.add_function(wrap_pyfunction!(enrich_swift_graph, m)?)?;
    m.add_function(wrap_pyfunction!(finalize_struct_graph, m)?)?;
    m.add_function(wrap_pyfunction!(prune_struct_shadow_graph, m)?)?;
    m.add_function(wrap_pyfunction!(init, m)?)?;
    m.add_function(wrap_pyfunction!(configure, m)?)?;
    m.add_function(wrap_pyfunction!(download, m)?)?;
    m.add_function(wrap_pyfunction!(download_all, m)?)?;
    m.add_function(wrap_pyfunction!(manifest_languages, m)?)?;
    m.add_function(wrap_pyfunction!(downloaded_languages, m)?)?;
    m.add_function(wrap_pyfunction!(clean_cache, m)?)?;
    m.add_function(wrap_pyfunction!(cache_dir, m)?)?;
    m.add_function(wrap_pyfunction!(build_line_window_chunks, m)?)?;
    m.add_function(wrap_pyfunction!(build_swift_chunks, m)?)?;
    m.add_function(wrap_pyfunction!(build_semantic_payload, m)?)?;
    m.add_function(wrap_pyfunction!(build_semantic_sync_plan, m)?)?;
    m.add_function(wrap_pyfunction!(build_codebase_embedding_rows, m)?)?;
    m.add_function(wrap_pyfunction!(build_semantic_index_round_plan, m)?)?;
    m.add_function(wrap_pyfunction!(build_semantic_index_driver_plan, m)?)?;
    m.add_function(wrap_pyfunction!(execute_codebase_embedding_upsert, m)?)?;
    m.add_function(wrap_pyfunction!(execute_semantic_index_driver, m)?)?;
    Ok(())
}
