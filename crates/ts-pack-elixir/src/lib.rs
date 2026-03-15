use rustler::{Encoder, Env, Error, NifResult, ResourceArc, Term};
use std::sync::Mutex;

mod atoms {
    rustler::atoms! {
        language_not_found,
        parse_error,
        nil,
    }
}

/// Wraps a tree-sitter Tree for safe sharing across the NIF boundary.
pub struct TreeResource(Mutex<tree_sitter::Tree>);

#[rustler::resource_impl]
impl rustler::Resource for TreeResource {}

#[rustler::nif]
fn available_languages() -> Vec<String> {
    ts_pack_core::available_languages()
}

#[rustler::nif]
fn has_language(name: String) -> bool {
    ts_pack_core::has_language(&name)
}

#[rustler::nif]
fn language_count() -> usize {
    ts_pack_core::language_count()
}

#[rustler::nif]
fn get_language_ptr(name: String) -> NifResult<u64> {
    let language = ts_pack_core::get_language(&name)
        .map_err(|_| Error::RaiseTerm(Box::new((atoms::language_not_found(), name.clone()))))?;
    let raw_ptr = language.into_raw();
    Ok(raw_ptr as u64)
}

#[rustler::nif]
fn parse_string(language: String, source: String) -> NifResult<ResourceArc<TreeResource>> {
    let tree = ts_pack_core::parse_string(&language, source.as_bytes())
        .map_err(|e| Error::RaiseTerm(Box::new((atoms::parse_error(), format!("{e}")))))?;
    Ok(ResourceArc::new(TreeResource(Mutex::new(tree))))
}

#[rustler::nif]
fn tree_root_node_type(tree: ResourceArc<TreeResource>) -> NifResult<String> {
    let guard = tree
        .0
        .lock()
        .map_err(|_| Error::RaiseTerm(Box::new((atoms::parse_error(), "lock poisoned".to_string()))))?;
    Ok(guard.root_node().kind().to_string())
}

#[rustler::nif]
fn tree_root_child_count(tree: ResourceArc<TreeResource>) -> NifResult<u32> {
    let guard = tree
        .0
        .lock()
        .map_err(|_| Error::RaiseTerm(Box::new((atoms::parse_error(), "lock poisoned".to_string()))))?;
    Ok(guard.root_node().named_child_count() as u32)
}

#[rustler::nif]
fn tree_contains_node_type(tree: ResourceArc<TreeResource>, node_type: String) -> NifResult<bool> {
    let guard = tree
        .0
        .lock()
        .map_err(|_| Error::RaiseTerm(Box::new((atoms::parse_error(), "lock poisoned".to_string()))))?;
    Ok(ts_pack_core::tree_contains_node_type(&guard, &node_type))
}

#[rustler::nif]
fn tree_has_error_nodes(tree: ResourceArc<TreeResource>) -> NifResult<bool> {
    let guard = tree
        .0
        .lock()
        .map_err(|_| Error::RaiseTerm(Box::new((atoms::parse_error(), "lock poisoned".to_string()))))?;
    Ok(ts_pack_core::tree_has_error_nodes(&guard))
}

// ---------------------------------------------------------------------------
// Process: unified API
// ---------------------------------------------------------------------------

/// Convert a serde_json::Value to an Elixir term (map, list, string, integer, float, boolean, nil).
fn json_to_term<'a>(env: Env<'a>, value: &serde_json::Value) -> Term<'a> {
    match value {
        serde_json::Value::Null => atoms::nil().encode(env),
        serde_json::Value::Bool(b) => b.encode(env),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                i.encode(env)
            } else if let Some(f) = n.as_f64() {
                f.encode(env)
            } else {
                atoms::nil().encode(env)
            }
        }
        serde_json::Value::String(s) => s.encode(env),
        serde_json::Value::Array(arr) => {
            let terms: Vec<Term<'a>> = arr.iter().map(|v| json_to_term(env, v)).collect();
            terms.encode(env)
        }
        serde_json::Value::Object(map) => {
            let pairs: Vec<(Term<'a>, Term<'a>)> =
                map.iter().map(|(k, v)| (k.encode(env), json_to_term(env, v))).collect();
            Term::map_from_pairs(env, &pairs).unwrap_or_else(|_| atoms::nil().encode(env))
        }
    }
}

/// Process source code and extract metadata + chunks as an Elixir map.
///
/// `config_json` is a JSON string with fields:
/// - `language` (string, required): the language name
/// - `chunk_max_size` (number, optional): maximum chunk size in bytes (default: 1500)
#[rustler::nif]
fn process<'a>(env: Env<'a>, source: String, config_json: String) -> NifResult<Term<'a>> {
    let core_config: ts_pack_core::ProcessConfig = serde_json::from_str(&config_json)
        .map_err(|e| Error::RaiseTerm(Box::new((atoms::parse_error(), format!("invalid config JSON: {e}")))))?;
    let result = ts_pack_core::process(&source, &core_config)
        .map_err(|e| Error::RaiseTerm(Box::new((atoms::parse_error(), format!("{e}")))))?;
    let value = serde_json::to_value(&result)
        .map_err(|e| Error::RaiseTerm(Box::new((atoms::parse_error(), format!("serialization failed: {e}")))))?;
    Ok(json_to_term(env, &value))
}

rustler::init!("Elixir.TreeSitterLanguagePack");
