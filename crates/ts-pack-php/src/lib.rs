//! PHP bindings for tree-sitter-language-pack.
//!
//! This module exposes the Rust core parsing API to PHP using ext-php-rs.
//!
//! # Architecture
//!
//! - All parsing logic is in the Rust core (ts-pack-core)
//! - PHP is a thin wrapper that adds language-specific features
//! - Zero duplication of core functionality

#![cfg_attr(windows, feature(abi_vectorcall))]

use ext_php_rs::prelude::*;

/// Get the library version.
///
/// # Returns
///
/// Version string in semver format (e.g., "1.0.0-rc.1")
///
/// # Example
///
/// ```php
/// $version = ts_pack_version();
/// echo "Version: $version\n";
/// ```
#[php_function]
pub fn ts_pack_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Get a list of all available language names.
///
/// # Returns
///
/// Array of language name strings sorted alphabetically.
///
/// # Example
///
/// ```php
/// $languages = ts_pack_available_languages();
/// foreach ($languages as $lang) {
///     echo "$lang\n";
/// }
/// ```
#[php_function]
pub fn ts_pack_available_languages() -> Vec<String> {
    tree_sitter_language_pack::available_languages()
}

/// Check whether a language is available.
///
/// # Arguments
///
/// * `name` - The language name to check.
///
/// # Returns
///
/// `true` if the language is available, `false` otherwise.
///
/// # Example
///
/// ```php
/// if (ts_pack_has_language("python")) {
///     echo "Python is available!\n";
/// }
/// ```
#[php_function]
pub fn ts_pack_has_language(name: String) -> bool {
    tree_sitter_language_pack::has_language(&name)
}

/// Detect language name from a file path or extension.
///
/// Returns null if the extension is not recognized.
#[php_function]
pub fn ts_pack_detect_language(path: String) -> Option<String> {
    tree_sitter_language_pack::detect_language_from_path(&path).map(String::from)
}

/// Detect language name from file content using shebang-based detection.
///
/// Returns null if the content does not contain a recognized shebang.
///
/// # Example
///
/// ```php
/// $lang = ts_pack_detect_language_from_content("#!/usr/bin/env python3\nprint('hello')\n");
/// echo "Detected: $lang\n"; // "python"
/// ```
#[php_function]
pub fn ts_pack_detect_language_from_content(content: String) -> Option<String> {
    tree_sitter_language_pack::detect_language_from_content(&content).map(String::from)
}

/// Detect language name from a bare file extension (without leading dot).
///
/// Returns null if the extension is not recognized.
///
/// # Example
///
/// ```php
/// $lang = ts_pack_detect_language_from_extension("py");
/// echo "Detected: $lang\n"; // "python"
/// ```
#[php_function]
pub fn ts_pack_detect_language_from_extension(ext: String) -> Option<String> {
    tree_sitter_language_pack::detect_language_from_extension(&ext).map(String::from)
}

/// Detect language name from a file path based on its extension.
///
/// Returns null if the extension is not recognized.
///
/// # Example
///
/// ```php
/// $lang = ts_pack_detect_language_from_path("/home/user/script.py");
/// echo "Detected: $lang\n"; // "python"
/// ```
#[php_function]
pub fn ts_pack_detect_language_from_path(path: String) -> Option<String> {
    tree_sitter_language_pack::detect_language_from_path(&path).map(String::from)
}

/// Returns extension ambiguity information for the given file extension as a JSON string.
///
/// Returns null if the extension is not ambiguous. When non-null, the JSON decodes to
/// an object with "assigned" (string) and "alternatives" (string[]) fields.
///
/// # Example
///
/// ```php
/// $info = ts_pack_extension_ambiguity("h");
/// $data = json_decode($info, true);
/// echo "Assigned: " . $data["assigned"] . "\n";
/// ```
#[php_function]
pub fn ts_pack_extension_ambiguity(ext: String) -> Option<String> {
    tree_sitter_language_pack::extension_ambiguity_json(&ext)
}

/// Returns the bundled highlights query for the given language, or null.
///
/// # Example
///
/// ```php
/// $query = ts_pack_get_highlights_query("python");
/// if ($query !== null) {
///     echo "Got highlights query (" . strlen($query) . " bytes)\n";
/// }
/// ```
#[php_function]
pub fn ts_pack_get_highlights_query(language: String) -> Option<String> {
    tree_sitter_language_pack::get_highlights_query(&language).map(String::from)
}

/// Returns the bundled injections query for the given language, or null.
///
/// # Example
///
/// ```php
/// $query = ts_pack_get_injections_query("markdown");
/// if ($query !== null) {
///     echo "Got injections query\n";
/// }
/// ```
#[php_function]
pub fn ts_pack_get_injections_query(language: String) -> Option<String> {
    tree_sitter_language_pack::get_injections_query(&language).map(String::from)
}

/// Returns the bundled locals query for the given language, or null.
///
/// # Example
///
/// ```php
/// $query = ts_pack_get_locals_query("python");
/// if ($query !== null) {
///     echo "Got locals query\n";
/// }
/// ```
#[php_function]
pub fn ts_pack_get_locals_query(language: String) -> Option<String> {
    tree_sitter_language_pack::get_locals_query(&language).map(String::from)
}

/// Get the number of available languages.
///
/// # Returns
///
/// The count of available languages as an integer.
///
/// # Example
///
/// ```php
/// $count = ts_pack_language_count();
/// echo "Available languages: $count\n";
/// ```
#[php_function]
pub fn ts_pack_language_count() -> i64 {
    tree_sitter_language_pack::language_count() as i64
}

/// Get a raw language pointer as an integer handle.
///
/// Returns the raw `TSLanguage` pointer cast to `i64`, which can be used by PHP
/// code to verify that a language is available and obtain its opaque handle.
///
/// # Arguments
///
/// * `name` - The language name to look up.
///
/// # Returns
///
/// The raw language pointer as an `i64` value.
///
/// # Throws
///
/// Throws an exception if the language is not available.
///
/// # Example
///
/// ```php
/// $langPtr = ts_pack_get_language("python");
/// echo "Got language pointer: $langPtr\n";
/// ```
#[php_function]
pub fn ts_pack_get_language(name: String) -> PhpResult<i64> {
    let lang = tree_sitter_language_pack::get_language(&name).map_err(|e| PhpException::default(format!("{e}")))?;
    Ok(lang.into_raw() as i64)
}

/// Parse source code and return an S-expression representation of the syntax tree.
///
/// # Arguments
///
/// * `language` - The language name to use for parsing.
/// * `source` - The source code to parse.
///
/// # Returns
///
/// The S-expression string representation of the parsed tree.
///
/// # Throws
///
/// Throws an exception if the language is not available or parsing fails.
///
/// # Example
///
/// ```php
/// $sexp = ts_pack_parse_string("python", "def hello(): pass");
/// echo "Tree: $sexp\n";
/// ```
#[php_function]
pub fn ts_pack_parse_string(language: String, source: String) -> PhpResult<String> {
    let tree = tree_sitter_language_pack::parse_string(&language, source.as_bytes())
        .map_err(|e| PhpException::default(format!("{e}")))?;
    Ok(tree_sitter_language_pack::tree_to_sexp(&tree))
}

/// Process source code and extract metadata + chunks as a JSON string.
///
/// The config JSON must contain at least `"language"`. Optional fields:
/// - `structure` (bool, default true): Extract structural items (functions, classes, etc.)
/// - `imports` (bool, default true): Extract import statements
/// - `exports` (bool, default true): Extract export statements
/// - `comments` (bool, default false): Extract comments
/// - `docstrings` (bool, default false): Extract docstrings
/// - `symbols` (bool, default false): Extract symbol definitions
/// - `diagnostics` (bool, default false): Include parse diagnostics
/// - `chunk_max_size` (int or null, default null): Maximum chunk size in bytes
///
/// # Arguments
///
/// * `source` - The source code to process.
/// * `config_json` - JSON string with processing configuration.
///
/// # Returns
///
/// JSON string with extraction results.
///
/// # Throws
///
/// Throws an exception if the config JSON is invalid, the language is unknown,
/// or processing fails.
///
/// # Example
///
/// ```php
/// $result = ts_pack_process("def hello(): pass", '{"language":"python"}');
/// $data = json_decode($result, true);
/// echo "Functions: " . count($data['structure']) . "\n";
/// ```
#[php_function]
pub fn ts_pack_process(source: String, config_json: String) -> PhpResult<String> {
    let core_config: tree_sitter_language_pack::ProcessConfig =
        serde_json::from_str(&config_json).map_err(|e| PhpException::default(format!("invalid config JSON: {e}")))?;

    let result =
        tree_sitter_language_pack::process(&source, &core_config).map_err(|e| PhpException::default(format!("{e}")))?;

    serde_json::to_string(&result).map_err(|e| PhpException::default(format!("serialization failed: {e}")))
}

/// Extract patterns from source code using a JSON configuration.
///
/// The config JSON must contain:
/// - `language` (string): the language name
/// - `patterns` (object): named patterns to run, each with a `query` field
///
/// # Arguments
///
/// * `source` - The source code to extract from.
/// * `config_json` - JSON string with extraction configuration.
///
/// # Returns
///
/// JSON string with extraction results.
///
/// # Throws
///
/// Throws an exception if the config JSON is invalid, the language is unknown,
/// or extraction fails.
///
/// # Example
///
/// ```php
/// $config = '{"language":"python","patterns":{"fns":{"query":"(function_definition name: (identifier) @fn_name)"}}}';
/// $result = ts_pack_extract("def hello(): pass", $config);
/// $data = json_decode($result, true);
/// ```
#[php_function]
pub fn ts_pack_extract(source: String, config_json: String) -> PhpResult<String> {
    let config: tree_sitter_language_pack::ExtractionConfig =
        serde_json::from_str(&config_json).map_err(|e| PhpException::default(format!("invalid config JSON: {e}")))?;

    let result = tree_sitter_language_pack::extract_patterns(&source, &config)
        .map_err(|e| PhpException::default(format!("{e}")))?;

    serde_json::to_string(&result).map_err(|e| PhpException::default(format!("serialization failed: {e}")))
}

/// Validate extraction patterns without running them.
///
/// The config JSON must contain:
/// - `language` (string): the language name
/// - `patterns` (object): named patterns to validate
///
/// # Arguments
///
/// * `config_json` - JSON string with extraction configuration.
///
/// # Returns
///
/// JSON string with validation results.
///
/// # Throws
///
/// Throws an exception if the config JSON is invalid or the language is unknown.
///
/// # Example
///
/// ```php
/// $config = '{"language":"python","patterns":{"fns":{"query":"(function_definition name: (identifier) @fn_name)"}}}';
/// $result = ts_pack_validate_extraction($config);
/// $data = json_decode($result, true);
/// echo $data['valid'] ? "Valid\n" : "Invalid\n";
/// ```
#[php_function]
pub fn ts_pack_validate_extraction(config_json: String) -> PhpResult<String> {
    let config: tree_sitter_language_pack::ExtractionConfig =
        serde_json::from_str(&config_json).map_err(|e| PhpException::default(format!("invalid config JSON: {e}")))?;

    let result =
        tree_sitter_language_pack::validate_extraction(&config).map_err(|e| PhpException::default(format!("{e}")))?;

    serde_json::to_string(&result).map_err(|e| PhpException::default(format!("serialization failed: {e}")))
}

/// Initialize the language pack with the given configuration (JSON string).
///
/// Applies cache directory settings and downloads specified languages/groups.
/// `config_json` should contain optional fields:
/// - `cache_dir` (string, optional): custom cache directory path
/// - `languages` (list, optional): language names to download
/// - `groups` (list, optional): language groups to download
///
/// # Arguments
///
/// * `config_json` - JSON string with configuration.
///
/// # Throws
///
/// Throws an exception if the config JSON is invalid or downloads fail.
///
/// # Example
///
/// ```php
/// ts_pack_init('{"languages":["python","rust"]}');
/// ```
#[php_function]
pub fn ts_pack_init(config_json: String) -> PhpResult<()> {
    let config: tree_sitter_language_pack::PackConfig =
        serde_json::from_str(&config_json).map_err(|e| PhpException::default(format!("invalid config JSON: {e}")))?;
    tree_sitter_language_pack::init(&config).map_err(|e| PhpException::default(format!("{e}")))
}

/// Apply download configuration without downloading anything.
///
/// Use this to set a custom cache directory before the first call to
/// [`ts_pack_get_language`] or any download function.
/// `config_json` should contain optional fields:
/// - `cache_dir` (string, optional): custom cache directory path
///
/// # Arguments
///
/// * `config_json` - JSON string with configuration.
///
/// # Throws
///
/// Throws an exception if the config JSON is invalid.
///
/// # Example
///
/// ```php
/// ts_pack_configure('{"cache_dir":"/tmp/parsers"}');
/// ```
#[php_function]
pub fn ts_pack_configure(config_json: String) -> PhpResult<()> {
    let config: tree_sitter_language_pack::PackConfig =
        serde_json::from_str(&config_json).map_err(|e| PhpException::default(format!("invalid config JSON: {e}")))?;
    tree_sitter_language_pack::configure(&config).map_err(|e| PhpException::default(format!("{e}")))
}

/// Download specific languages to the local cache.
///
/// Returns the number of newly downloaded languages (already cached languages
/// are not counted).
///
/// # Arguments
///
/// * `names` - Array of language names to download.
///
/// # Returns
///
/// Integer count of newly downloaded languages.
///
/// # Throws
///
/// Throws an exception if any language is not available or download fails.
///
/// # Example
///
/// ```php
/// $count = ts_pack_download(["python", "rust", "typescript"]);
/// echo "Downloaded $count new languages\n";
/// ```
#[php_function]
pub fn ts_pack_download(names: Vec<String>) -> PhpResult<i64> {
    let refs: Vec<&str> = names.iter().map(String::as_str).collect();
    tree_sitter_language_pack::download(&refs)
        .map(|count| count as i64)
        .map_err(|e| PhpException::default(format!("{e}")))
}

/// Download all available languages from the remote manifest.
///
/// Returns the number of newly downloaded languages.
///
/// # Returns
///
/// Integer count of newly downloaded languages.
///
/// # Throws
///
/// Throws an exception if the manifest cannot be fetched or a download fails.
///
/// # Example
///
/// ```php
/// $count = ts_pack_download_all();
/// echo "Downloaded $count languages\n";
/// ```
#[php_function]
pub fn ts_pack_download_all() -> PhpResult<i64> {
    tree_sitter_language_pack::download_all()
        .map(|count| count as i64)
        .map_err(|e| PhpException::default(format!("{e}")))
}

/// Return all language names available in the remote manifest (248).
///
/// Fetches (and caches) the remote manifest to discover the full list of
/// downloadable languages.
///
/// # Returns
///
/// Array of language names sorted alphabetically.
///
/// # Throws
///
/// Throws an exception if the manifest cannot be fetched.
///
/// # Example
///
/// ```php
/// $langs = ts_pack_manifest_languages();
/// echo count($langs) . " languages available for download\n";
/// ```
#[php_function]
pub fn ts_pack_manifest_languages() -> PhpResult<Vec<String>> {
    tree_sitter_language_pack::manifest_languages().map_err(|e| PhpException::default(format!("{e}")))
}

/// Return languages that are already downloaded and cached locally.
///
/// Does not perform any network requests. Returns an empty array if the
/// cache directory does not exist or cannot be read.
///
/// # Returns
///
/// Array of cached language names sorted alphabetically.
///
/// # Example
///
/// ```php
/// $cached = ts_pack_downloaded_languages();
/// echo count($cached) . " languages already cached\n";
/// ```
#[php_function]
pub fn ts_pack_downloaded_languages() -> Vec<String> {
    tree_sitter_language_pack::downloaded_languages()
}

/// Delete all cached parser shared libraries.
///
/// Resets the cache registration so the next call to ts_pack_get_language or
/// a download function will re-register the (now empty) cache directory.
///
/// # Throws
///
/// Throws an exception if the cache directory cannot be removed.
///
/// # Example
///
/// ```php
/// ts_pack_clean_cache();
/// echo "Cache cleared\n";
/// ```
#[php_function]
pub fn ts_pack_clean_cache() -> PhpResult<()> {
    tree_sitter_language_pack::clean_cache().map_err(|e| PhpException::default(format!("{e}")))
}

/// Return the effective cache directory path.
///
/// This is either the custom path set via ts_pack_configure/ts_pack_init or the
/// default: `~/.cache/tree-sitter-language-pack/v{version}/libs/`
///
/// # Returns
///
/// String path to the cache directory.
///
/// # Throws
///
/// Throws an exception if the system cache directory cannot be determined.
///
/// # Example
///
/// ```php
/// $dir = ts_pack_cache_dir();
/// echo "Cache directory: $dir\n";
/// ```
#[php_function]
pub fn ts_pack_cache_dir() -> PhpResult<String> {
    tree_sitter_language_pack::cache_dir()
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|e| PhpException::default(format!("{e}")))
}

/// tree-sitter-language-pack PHP extension module.
#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .function(wrap_function!(ts_pack_version))
        .function(wrap_function!(ts_pack_available_languages))
        .function(wrap_function!(ts_pack_has_language))
        .function(wrap_function!(ts_pack_detect_language))
        .function(wrap_function!(ts_pack_detect_language_from_content))
        .function(wrap_function!(ts_pack_detect_language_from_extension))
        .function(wrap_function!(ts_pack_detect_language_from_path))
        .function(wrap_function!(ts_pack_extension_ambiguity))
        .function(wrap_function!(ts_pack_get_highlights_query))
        .function(wrap_function!(ts_pack_get_injections_query))
        .function(wrap_function!(ts_pack_get_locals_query))
        .function(wrap_function!(ts_pack_language_count))
        .function(wrap_function!(ts_pack_get_language))
        .function(wrap_function!(ts_pack_parse_string))
        .function(wrap_function!(ts_pack_process))
        .function(wrap_function!(ts_pack_extract))
        .function(wrap_function!(ts_pack_validate_extraction))
        .function(wrap_function!(ts_pack_init))
        .function(wrap_function!(ts_pack_configure))
        .function(wrap_function!(ts_pack_download))
        .function(wrap_function!(ts_pack_download_all))
        .function(wrap_function!(ts_pack_manifest_languages))
        .function(wrap_function!(ts_pack_downloaded_languages))
        .function(wrap_function!(ts_pack_clean_cache))
        .function(wrap_function!(ts_pack_cache_dir))
}
