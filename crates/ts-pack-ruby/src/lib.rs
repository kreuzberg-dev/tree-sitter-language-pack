use magnus::{Error, IntoValue, Ruby, Value as RbValue, function, method, prelude::*};
use std::sync::Mutex;

/// Wraps a tree-sitter Tree for safe sharing across the Ruby boundary.
#[magnus::wrap(class = "TreeSitterLanguagePack::Tree")]
struct TreeWrapper(Mutex<tree_sitter::Tree>);

/// Helper to create a runtime error from instance methods where `&Ruby` is not available.
fn lock_error() -> Error {
    // SAFETY: This is called from Ruby-invoked methods, so the Ruby VM is active.
    let ruby = unsafe { Ruby::get_unchecked() };
    Error::new(ruby.exception_runtime_error(), "lock poisoned")
}

impl TreeWrapper {
    fn root_node_type(&self) -> Result<String, Error> {
        let guard = self.0.lock().map_err(|_| lock_error())?;
        Ok(guard.root_node().kind().to_string())
    }

    fn root_child_count(&self) -> Result<usize, Error> {
        let guard = self.0.lock().map_err(|_| lock_error())?;
        Ok(guard.root_node().named_child_count())
    }

    fn contains_node_type(&self, node_type: String) -> Result<bool, Error> {
        let guard = self.0.lock().map_err(|_| lock_error())?;
        Ok(tree_sitter_language_pack::tree_contains_node_type(&guard, &node_type))
    }

    fn has_error_nodes(&self) -> Result<bool, Error> {
        let guard = self.0.lock().map_err(|_| lock_error())?;
        Ok(tree_sitter_language_pack::tree_has_error_nodes(&guard))
    }
}

fn available_languages() -> Vec<String> {
    tree_sitter_language_pack::available_languages()
}

fn has_language(name: String) -> bool {
    tree_sitter_language_pack::has_language(&name)
}

fn detect_language(path: String) -> Option<String> {
    tree_sitter_language_pack::detect_language_from_path(&path).map(String::from)
}

fn detect_language_from_content(content: String) -> Option<String> {
    tree_sitter_language_pack::detect_language_from_content(&content).map(String::from)
}

fn detect_language_from_extension(ext: String) -> Option<String> {
    tree_sitter_language_pack::detect_language_from_extension(&ext).map(String::from)
}

fn detect_language_from_path(path: String) -> Option<String> {
    tree_sitter_language_pack::detect_language_from_path(&path).map(String::from)
}

/// Returns extension ambiguity information as a JSON string, or nil.
fn extension_ambiguity(ext: String) -> Option<String> {
    tree_sitter_language_pack::extension_ambiguity_json(&ext)
}

fn get_highlights_query(language: String) -> Option<String> {
    tree_sitter_language_pack::get_highlights_query(&language).map(String::from)
}

fn get_injections_query(language: String) -> Option<String> {
    tree_sitter_language_pack::get_injections_query(&language).map(String::from)
}

fn get_locals_query(language: String) -> Option<String> {
    tree_sitter_language_pack::get_locals_query(&language).map(String::from)
}

fn language_count() -> usize {
    tree_sitter_language_pack::language_count()
}

fn get_language_ptr(ruby: &Ruby, name: String) -> Result<u64, Error> {
    let language = tree_sitter_language_pack::get_language(&name)
        .map_err(|_| Error::new(ruby.exception_runtime_error(), format!("language not found: {name}")))?;
    let raw_ptr = language.into_raw();
    Ok(raw_ptr as u64)
}

fn parse_string(ruby: &Ruby, language: String, source: String) -> Result<TreeWrapper, Error> {
    let tree = tree_sitter_language_pack::parse_string(&language, source.as_bytes())
        .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("{e}")))?;
    Ok(TreeWrapper(Mutex::new(tree)))
}

/// Convert a serde_json::Value to a native Ruby object (Hash, Array, String, Integer, Float, true/false, nil).
fn json_value_to_ruby(ruby: &Ruby, value: &serde_json::Value) -> Result<RbValue, Error> {
    Ok(match value {
        serde_json::Value::Null => ruby.qnil().as_value(),
        serde_json::Value::Bool(b) => (*b).into_value_with(ruby),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                i.into_value_with(ruby)
            } else if let Some(u) = n.as_u64() {
                u.into_value_with(ruby)
            } else if let Some(f) = n.as_f64() {
                f.into_value_with(ruby)
            } else {
                ruby.qnil().as_value()
            }
        }
        serde_json::Value::String(s) => s.as_str().into_value_with(ruby),
        serde_json::Value::Array(arr) => {
            let ary = ruby.ary_new_capa(arr.len());
            for item in arr {
                ary.push(json_value_to_ruby(ruby, item)?)?;
            }
            ary.as_value()
        }
        serde_json::Value::Object(map) => {
            let hash = ruby.hash_new();
            for (k, v) in map {
                hash.aset(k.as_str(), json_value_to_ruby(ruby, v)?)?;
            }
            hash.as_value()
        }
    })
}

/// Unified process method that accepts a JSON config string and returns a native Ruby Hash.
///
/// The config JSON must contain at least `"language"`. Optional fields:
/// - `structure`, `imports`, `exports`, `comments`, `docstrings`, `symbols`, `diagnostics` (booleans, default true)
/// - `chunk_max_size` (integer or null, default null meaning no chunking)
fn process(ruby: &Ruby, source: String, config_json: String) -> Result<RbValue, Error> {
    let core_config: tree_sitter_language_pack::ProcessConfig = serde_json::from_str(&config_json)
        .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("invalid config JSON: {e}")))?;

    let result = tree_sitter_language_pack::process(&source, &core_config)
        .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("{e}")))?;

    let json_value = serde_json::to_value(&result)
        .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("serialization failed: {e}")))?;
    json_value_to_ruby(ruby, &json_value)
}

/// Extract patterns from source code using a JSON configuration.
///
/// The config JSON must contain:
/// - `language` (string): the language name
/// - `patterns` (object): named patterns to run, each with a `query` field
///
/// Returns a native Ruby Hash with extraction results.
fn extract(ruby: &Ruby, source: String, config_json: String) -> Result<RbValue, Error> {
    let config: tree_sitter_language_pack::ExtractionConfig = serde_json::from_str(&config_json)
        .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("invalid config JSON: {e}")))?;

    let result = tree_sitter_language_pack::extract_patterns(&source, &config)
        .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("{e}")))?;

    let json_value = serde_json::to_value(&result)
        .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("serialization failed: {e}")))?;
    json_value_to_ruby(ruby, &json_value)
}

/// Validate extraction patterns without running them.
///
/// The config JSON must contain:
/// - `language` (string): the language name
/// - `patterns` (object): named patterns to validate
///
/// Returns a native Ruby Hash with validation results.
fn validate_extraction(ruby: &Ruby, config_json: String) -> Result<RbValue, Error> {
    let config: tree_sitter_language_pack::ExtractionConfig = serde_json::from_str(&config_json)
        .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("invalid config JSON: {e}")))?;

    let result = tree_sitter_language_pack::validate_extraction(&config)
        .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("{e}")))?;

    let json_value = serde_json::to_value(&result)
        .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("serialization failed: {e}")))?;
    json_value_to_ruby(ruby, &json_value)
}

/// Initialize the language pack with configuration.
///
/// Accepts a JSON string with optional fields:
/// - `cache_dir` (string): override default cache directory
/// - `languages` (array): language names to pre-download
/// - `groups` (array): language groups to pre-download
fn rb_init(ruby: &Ruby, config_json: String) -> Result<(), Error> {
    let config: tree_sitter_language_pack::PackConfig = serde_json::from_str(&config_json)
        .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("invalid config JSON: {e}")))?;

    tree_sitter_language_pack::init(&config).map_err(|e| Error::new(ruby.exception_runtime_error(), format!("{e}")))
}

/// Configure the language pack without downloading.
///
/// Accepts a JSON string with optional fields:
/// - `cache_dir` (string): override default cache directory
fn rb_configure(ruby: &Ruby, config_json: String) -> Result<(), Error> {
    let config: tree_sitter_language_pack::PackConfig = serde_json::from_str(&config_json)
        .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("invalid config JSON: {e}")))?;

    tree_sitter_language_pack::configure(&config)
        .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("{e}")))
}

/// Download specific languages to the cache.
///
/// Returns the number of newly downloaded languages.
fn rb_download(ruby: &Ruby, names: Vec<String>) -> Result<usize, Error> {
    let refs: Vec<&str> = names.iter().map(String::as_str).collect();
    tree_sitter_language_pack::download(&refs).map_err(|e| Error::new(ruby.exception_runtime_error(), format!("{e}")))
}

/// Download all available languages from the remote manifest.
///
/// Returns the number of newly downloaded languages.
fn rb_download_all(ruby: &Ruby) -> Result<usize, Error> {
    tree_sitter_language_pack::download_all().map_err(|e| Error::new(ruby.exception_runtime_error(), format!("{e}")))
}

/// Get all language names available in the remote manifest.
fn rb_manifest_languages(ruby: &Ruby) -> Result<Vec<String>, Error> {
    tree_sitter_language_pack::manifest_languages()
        .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("{e}")))
}

/// Get all languages that are already downloaded and cached locally.
fn rb_downloaded_languages() -> Vec<String> {
    tree_sitter_language_pack::downloaded_languages()
}

/// Delete all cached parser shared libraries.
fn rb_clean_cache(ruby: &Ruby) -> Result<(), Error> {
    tree_sitter_language_pack::clean_cache().map_err(|e| Error::new(ruby.exception_runtime_error(), format!("{e}")))
}

/// Get the effective cache directory path as a string.
fn rb_cache_dir(ruby: &Ruby) -> Result<String, Error> {
    tree_sitter_language_pack::cache_dir()
        .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("{e}")))
        .and_then(|path| {
            path.to_str()
                .ok_or_else(|| Error::new(ruby.exception_runtime_error(), "cache path is not valid UTF-8"))
                .map(|s| s.to_string())
        })
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("TreeSitterLanguagePack")?;

    // Registry and parsing functions
    module.define_module_function("available_languages", function!(available_languages, 0))?;
    module.define_module_function("has_language", function!(has_language, 1))?;
    module.define_module_function("detect_language", function!(detect_language, 1))?;
    module.define_module_function(
        "detect_language_from_content",
        function!(detect_language_from_content, 1),
    )?;
    module.define_module_function(
        "detect_language_from_extension",
        function!(detect_language_from_extension, 1),
    )?;
    module.define_module_function("detect_language_from_path", function!(detect_language_from_path, 1))?;
    module.define_module_function("extension_ambiguity", function!(extension_ambiguity, 1))?;
    module.define_module_function("get_highlights_query", function!(get_highlights_query, 1))?;
    module.define_module_function("get_injections_query", function!(get_injections_query, 1))?;
    module.define_module_function("get_locals_query", function!(get_locals_query, 1))?;
    module.define_module_function("language_count", function!(language_count, 0))?;
    module.define_module_function("get_language_ptr", function!(get_language_ptr, 1))?;
    module.define_module_function("parse_string", function!(parse_string, 2))?;
    module.define_module_function("process", function!(process, 2))?;
    module.define_module_function("extract", function!(extract, 2))?;
    module.define_module_function("validate_extraction", function!(validate_extraction, 1))?;

    // Download API functions
    module.define_module_function("init", function!(rb_init, 1))?;
    module.define_module_function("configure", function!(rb_configure, 1))?;
    module.define_module_function("download", function!(rb_download, 1))?;
    module.define_module_function("download_all", function!(rb_download_all, 0))?;
    module.define_module_function("manifest_languages", function!(rb_manifest_languages, 0))?;
    module.define_module_function("downloaded_languages", function!(rb_downloaded_languages, 0))?;
    module.define_module_function("clean_cache", function!(rb_clean_cache, 0))?;
    module.define_module_function("cache_dir", function!(rb_cache_dir, 0))?;

    let tree_class = module.define_class("Tree", ruby.class_object())?;
    tree_class.define_method("root_node_type", method!(TreeWrapper::root_node_type, 0))?;
    tree_class.define_method("root_child_count", method!(TreeWrapper::root_child_count, 0))?;
    tree_class.define_method("contains_node_type", method!(TreeWrapper::contains_node_type, 1))?;
    tree_class.define_method("has_error_nodes", method!(TreeWrapper::has_error_nodes, 0))?;

    Ok(())
}
