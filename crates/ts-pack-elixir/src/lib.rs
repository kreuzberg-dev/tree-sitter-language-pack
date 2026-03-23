use rustler::{Encoder, Env, Error, NifResult, ResourceArc, Term};
use std::sync::Mutex;

mod atoms {
    rustler::atoms! {
        language_not_found,
        parse_error,
        download_error,
        nil,
    }
}

/// Wraps a tree-sitter Tree for safe sharing across the NIF boundary.
pub struct TreeResource(Mutex<tree_sitter::Tree>);

#[rustler::resource_impl]
impl rustler::Resource for TreeResource {}

#[rustler::nif]
fn available_languages() -> Vec<String> {
    tree_sitter_language_pack::available_languages()
}

#[rustler::nif]
fn has_language(name: String) -> bool {
    tree_sitter_language_pack::has_language(&name)
}

#[rustler::nif]
fn detect_language(path: String) -> Option<String> {
    tree_sitter_language_pack::detect_language_from_path(&path).map(String::from)
}

#[rustler::nif]
fn language_count() -> usize {
    tree_sitter_language_pack::language_count()
}

#[rustler::nif]
fn get_language_ptr(name: String) -> NifResult<u64> {
    let language = tree_sitter_language_pack::get_language(&name)
        .map_err(|_| Error::RaiseTerm(Box::new((atoms::language_not_found(), name.clone()))))?;
    let raw_ptr = language.into_raw();
    Ok(raw_ptr as u64)
}

#[rustler::nif]
fn parse_string(language: String, source: String) -> NifResult<ResourceArc<TreeResource>> {
    let tree = tree_sitter_language_pack::parse_string(&language, source.as_bytes())
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
    Ok(tree_sitter_language_pack::tree_contains_node_type(&guard, &node_type))
}

#[rustler::nif]
fn tree_has_error_nodes(tree: ResourceArc<TreeResource>) -> NifResult<bool> {
    let guard = tree
        .0
        .lock()
        .map_err(|_| Error::RaiseTerm(Box::new((atoms::parse_error(), "lock poisoned".to_string()))))?;
    Ok(tree_sitter_language_pack::tree_has_error_nodes(&guard))
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

// ---------------------------------------------------------------------------
// Download API
// ---------------------------------------------------------------------------

/// Initialize the language pack with the given configuration (JSON string).
///
/// Applies cache directory settings and downloads specified languages/groups.
/// `config_json` should contain optional fields:
/// - `cache_dir` (string, optional): custom cache directory path
/// - `languages` (list, optional): language names to download
/// - `groups` (list, optional): language groups to download
#[rustler::nif(schedule = "DirtyIo")]
fn init(config_json: String) -> NifResult<()> {
    let config: tree_sitter_language_pack::PackConfig = serde_json::from_str(&config_json)
        .map_err(|e| Error::RaiseTerm(Box::new((atoms::parse_error(), format!("invalid config JSON: {e}")))))?;
    tree_sitter_language_pack::init(&config)
        .map_err(|e| Error::RaiseTerm(Box::new((atoms::download_error(), format!("{e}")))))
}

/// Apply download configuration without downloading anything.
///
/// Use this to set a custom cache directory before the first call to
/// [`get_language`] or any download function.
/// `config_json` should contain optional fields:
/// - `cache_dir` (string, optional): custom cache directory path
#[rustler::nif]
fn configure(config_json: String) -> NifResult<()> {
    let config: tree_sitter_language_pack::PackConfig = serde_json::from_str(&config_json)
        .map_err(|e| Error::RaiseTerm(Box::new((atoms::parse_error(), format!("invalid config JSON: {e}")))))?;
    tree_sitter_language_pack::configure(&config)
        .map_err(|e| Error::RaiseTerm(Box::new((atoms::download_error(), format!("{e}")))))
}

/// Download specific languages to the local cache.
///
/// Returns the number of newly downloaded languages.
#[rustler::nif(schedule = "DirtyIo")]
fn download(names: Vec<String>) -> NifResult<usize> {
    let refs: Vec<&str> = names.iter().map(String::as_str).collect();
    tree_sitter_language_pack::download(&refs)
        .map_err(|e| Error::RaiseTerm(Box::new((atoms::download_error(), format!("{e}")))))
}

/// Download all available languages from the remote manifest.
///
/// Returns the number of newly downloaded languages.
#[rustler::nif(schedule = "DirtyIo")]
fn download_all() -> NifResult<usize> {
    tree_sitter_language_pack::download_all()
        .map_err(|e| Error::RaiseTerm(Box::new((atoms::download_error(), format!("{e}")))))
}

/// Return all language names available in the remote manifest.
///
/// Fetches (and caches) the remote manifest to discover the full list of
/// downloadable languages.
#[rustler::nif(schedule = "DirtyIo")]
fn manifest_languages() -> NifResult<Vec<String>> {
    tree_sitter_language_pack::manifest_languages()
        .map_err(|e| Error::RaiseTerm(Box::new((atoms::download_error(), format!("{e}")))))
}

/// Return languages that are already downloaded and cached locally.
///
/// Does not perform any network requests.
#[rustler::nif]
fn downloaded_languages() -> Vec<String> {
    tree_sitter_language_pack::downloaded_languages()
}

/// Delete all cached parser shared libraries.
///
/// Resets the cache registration so the next call to get_language or
/// a download function will re-register the (now empty) cache directory.
#[rustler::nif(schedule = "DirtyIo")]
fn clean_cache() -> NifResult<()> {
    tree_sitter_language_pack::clean_cache()
        .map_err(|e| Error::RaiseTerm(Box::new((atoms::download_error(), format!("{e}")))))
}

/// Return the effective cache directory path as a string.
///
/// This is either the custom path set via configure/init or the default.
#[rustler::nif(schedule = "DirtyIo")]
fn cache_dir() -> NifResult<String> {
    tree_sitter_language_pack::cache_dir()
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|e| Error::RaiseTerm(Box::new((atoms::download_error(), format!("{e}")))))
}

/// Process source code and extract metadata + chunks as an Elixir map.
///
/// `config_json` is a JSON string with fields:
/// - `language` (string, required): the language name
/// - `chunk_max_size` (number, optional): maximum chunk size in bytes (default: 1500)
#[rustler::nif]
fn process<'a>(env: Env<'a>, source: String, config_json: String) -> NifResult<Term<'a>> {
    let core_config: tree_sitter_language_pack::ProcessConfig = serde_json::from_str(&config_json)
        .map_err(|e| Error::RaiseTerm(Box::new((atoms::parse_error(), format!("invalid config JSON: {e}")))))?;
    let result = tree_sitter_language_pack::process(&source, &core_config)
        .map_err(|e| Error::RaiseTerm(Box::new((atoms::parse_error(), format!("{e}")))))?;
    let value = serde_json::to_value(&result)
        .map_err(|e| Error::RaiseTerm(Box::new((atoms::parse_error(), format!("serialization failed: {e}")))))?;
    Ok(json_to_term(env, &value))
}

rustler::init!("Elixir.TreeSitterLanguagePack");
