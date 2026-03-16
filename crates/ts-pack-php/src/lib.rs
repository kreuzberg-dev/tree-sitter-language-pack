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

/// tree-sitter-language-pack PHP extension module.
#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .function(wrap_function!(ts_pack_version))
        .function(wrap_function!(ts_pack_available_languages))
        .function(wrap_function!(ts_pack_has_language))
        .function(wrap_function!(ts_pack_language_count))
        .function(wrap_function!(ts_pack_get_language))
        .function(wrap_function!(ts_pack_parse_string))
        .function(wrap_function!(ts_pack_process))
}
