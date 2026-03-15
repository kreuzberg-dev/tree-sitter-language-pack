use magnus::{Error, Ruby, function, method, prelude::*};
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
        Ok(ts_pack_core::tree_contains_node_type(&guard, &node_type))
    }

    fn has_error_nodes(&self) -> Result<bool, Error> {
        let guard = self.0.lock().map_err(|_| lock_error())?;
        Ok(ts_pack_core::tree_has_error_nodes(&guard))
    }
}

fn available_languages() -> Vec<String> {
    ts_pack_core::available_languages()
}

fn has_language(name: String) -> bool {
    ts_pack_core::has_language(&name)
}

fn language_count() -> usize {
    ts_pack_core::language_count()
}

fn get_language_ptr(ruby: &Ruby, name: String) -> Result<u64, Error> {
    let language = ts_pack_core::get_language(&name)
        .map_err(|_| Error::new(ruby.exception_runtime_error(), format!("language not found: {name}")))?;
    let raw_ptr = language.into_raw();
    Ok(raw_ptr as u64)
}

fn parse_string(ruby: &Ruby, language: String, source: String) -> Result<TreeWrapper, Error> {
    let tree = ts_pack_core::parse_string(&language, source.as_bytes())
        .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("{e}")))?;
    Ok(TreeWrapper(Mutex::new(tree)))
}

/// Convert a serde_json::Value to a Ruby value (Hash, Array, String, Integer, Float, true/false, nil).
fn json_value_to_ruby(ruby: &Ruby, value: &serde_json::Value) -> Result<magnus::Value, Error> {
    match value {
        serde_json::Value::Null => Ok(ruby.qnil().as_value()),
        serde_json::Value::Bool(b) => {
            if *b {
                Ok(ruby.qtrue().as_value())
            } else {
                Ok(ruby.qfalse().as_value())
            }
        }
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(ruby.integer_from_i64(i).as_value())
            } else if let Some(f) = n.as_f64() {
                Ok(ruby.float_from_f64(f).as_value())
            } else {
                Ok(ruby.qnil().as_value())
            }
        }
        serde_json::Value::String(s) => Ok(ruby.str_new(s).as_value()),
        serde_json::Value::Array(arr) => {
            let r_arr = ruby.ary_new_capa(arr.len());
            for v in arr {
                r_arr.push(json_value_to_ruby(ruby, v)?)?;
            }
            Ok(r_arr.as_value())
        }
        serde_json::Value::Object(map) => {
            let hash = ruby.hash_new();
            for (k, v) in map {
                hash.aset(ruby.str_new(k), json_value_to_ruby(ruby, v)?)?;
            }
            Ok(hash.as_value())
        }
    }
}

fn process_legacy(ruby: &Ruby, source: String, language: String) -> Result<magnus::Value, Error> {
    let intel = ts_pack_core::process(&source, &language)
        .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("{e}")))?;
    let value = serde_json::to_value(&intel)
        .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("serialization failed: {e}")))?;
    json_value_to_ruby(ruby, &value)
}

fn process_and_chunk(
    ruby: &Ruby,
    source: String,
    language: String,
    max_chunk_size: usize,
) -> Result<magnus::Value, Error> {
    let (intel, chunks) = ts_pack_core::process_and_chunk(&source, &language, max_chunk_size)
        .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("{e}")))?;
    let result = serde_json::json!({
        "intelligence": intel,
        "chunks": chunks,
    });
    json_value_to_ruby(ruby, &result)
}

/// Unified process method that accepts a JSON config string and returns a JSON result string.
///
/// The config JSON must contain at least `"language"`. Optional fields:
/// - `structure`, `imports`, `exports`, `comments`, `docstrings`, `symbols`, `diagnostics` (booleans, default true)
/// - `chunk_max_size` (integer or null, default null meaning no chunking)
///
/// When `chunk_max_size` is set, the result includes both intelligence and chunks.
/// When it is null/absent, only intelligence is returned (with an empty chunks array).
fn process(ruby: &Ruby, source: String, config_json: String) -> Result<String, Error> {
    let config: serde_json::Value = serde_json::from_str(&config_json)
        .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("invalid config JSON: {e}")))?;

    let language = config
        .get("language")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::new(ruby.exception_runtime_error(), "config must contain 'language' string field"))?;

    let chunk_max_size = config.get("chunk_max_size").and_then(|v| v.as_u64()).map(|v| v as usize);

    let result = if let Some(max_size) = chunk_max_size {
        let (intel, chunks) = ts_pack_core::process_and_chunk(&source, language, max_size)
            .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("{e}")))?;
        serde_json::json!({
            "metadata": intel,
            "chunks": chunks,
        })
    } else {
        let intel = ts_pack_core::process(&source, language)
            .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("{e}")))?;
        serde_json::json!({
            "metadata": intel,
            "chunks": [],
        })
    };

    serde_json::to_string(&result)
        .map_err(|e| Error::new(ruby.exception_runtime_error(), format!("serialization failed: {e}")))
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("TreeSitterLanguagePack")?;

    module.define_module_function("available_languages", function!(available_languages, 0))?;
    module.define_module_function("has_language", function!(has_language, 1))?;
    module.define_module_function("language_count", function!(language_count, 0))?;
    module.define_module_function("get_language_ptr", function!(get_language_ptr, 1))?;
    module.define_module_function("parse_string", function!(parse_string, 2))?;
    module.define_module_function("process", function!(process, 2))?;
    module.define_module_function("process_legacy", function!(process_legacy, 2))?;
    module.define_module_function("process_and_chunk", function!(process_and_chunk, 3))?;

    let tree_class = module.define_class("Tree", ruby.class_object())?;
    tree_class.define_method("root_node_type", method!(TreeWrapper::root_node_type, 0))?;
    tree_class.define_method("root_child_count", method!(TreeWrapper::root_child_count, 0))?;
    tree_class.define_method("contains_node_type", method!(TreeWrapper::contains_node_type, 1))?;
    tree_class.define_method("has_error_nodes", method!(TreeWrapper::has_error_nodes, 0))?;

    Ok(())
}
