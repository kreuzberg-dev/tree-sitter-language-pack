//! # tree-sitter-language-pack
//!
//! Pre-compiled tree-sitter grammars for 170+ programming languages with
//! a unified API for parsing, analysis, and intelligent code chunking.
//!
//! ## Quick Start
//!
//! ```no_run
//! use ts_pack_core::{ProcessConfig, available_languages, has_language, process};
//!
//! // Check available languages
//! let langs = available_languages();
//! assert!(has_language("python"));
//!
//! // Process source code
//! let config = ProcessConfig::new("python").all();
//! let result = process("def hello(): pass", &config).unwrap();
//! println!("Language: {}", result.language);
//! println!("Functions: {}", result.structure.len());
//! ```
//!
//! ## Modules
//!
//! - [`registry`] - Thread-safe language registry for parser lookup
//! - [`intel`] - Source code intelligence extraction (structure, imports, exports, etc.)
//! - [`parse`] - Low-level tree-sitter parsing utilities
//! - [`node`] - Tree node traversal and information extraction
//! - [`query`] - Tree-sitter query execution
//! - [`text_splitter`] - Syntax-aware code chunking
//! - [`process_config`] - Configuration for the `process` pipeline
//! - [`error`] - Error types

pub mod error;
pub mod intel;
pub mod node;
pub mod parse;
pub mod process_config;
pub mod query;
pub mod registry;
pub mod text_splitter;

#[cfg(feature = "config")]
pub mod config;
#[cfg(feature = "config")]
pub mod definitions;
#[cfg(feature = "download")]
pub mod download;

pub use error::Error;
pub use intel::types::{
    ChunkContext, CodeChunk, CommentInfo, CommentKind, Diagnostic, DiagnosticSeverity, DocSection, DocstringFormat,
    DocstringInfo, ExportInfo, ExportKind, FileMetrics, ImportInfo, ProcessResult, Span, StructureItem, StructureKind,
    SymbolInfo, SymbolKind,
};
pub use node::{NodeInfo, extract_text, find_nodes_by_type, named_children_info, node_info_from_node, root_node_info};
pub use parse::{parse_string, tree_contains_node_type, tree_error_count, tree_has_error_nodes, tree_to_sexp};
pub use process_config::ProcessConfig;
pub use query::{QueryMatch, run_query};
pub use registry::LanguageRegistry;
pub use text_splitter::split_code;
pub use tree_sitter::{Language, Parser, Tree};

#[cfg(feature = "download")]
pub use download::DownloadManager;

static REGISTRY: std::sync::LazyLock<LanguageRegistry> = std::sync::LazyLock::new(LanguageRegistry::new);

/// Get a tree-sitter [`Language`] by name using the global registry.
///
/// Resolves language aliases (e.g., `"shell"` maps to `"bash"`).
///
/// # Errors
///
/// Returns [`Error::LanguageNotFound`] if the language name is not recognized.
///
/// # Example
///
/// ```no_run
/// use ts_pack_core::get_language;
///
/// let lang = get_language("python").unwrap();
/// // Use the Language with a tree-sitter Parser
/// let mut parser = tree_sitter::Parser::new();
/// parser.set_language(&lang).unwrap();
/// let tree = parser.parse("x = 1", None).unwrap();
/// assert_eq!(tree.root_node().kind(), "module");
/// ```
pub fn get_language(name: &str) -> Result<Language, Error> {
    REGISTRY.get_language(name)
}

/// Get a tree-sitter [`Parser`] pre-configured for the given language.
///
/// This is a convenience function that calls [`get_language`] and configures
/// a new parser in one step.
///
/// # Errors
///
/// Returns [`Error::LanguageNotFound`] if the language is not recognized, or
/// [`Error::ParserSetup`] if the language cannot be applied to the parser.
///
/// # Example
///
/// ```no_run
/// use ts_pack_core::get_parser;
///
/// let mut parser = get_parser("rust").unwrap();
/// let tree = parser.parse("fn main() {}", None).unwrap();
/// assert!(!tree.root_node().has_error());
/// ```
pub fn get_parser(name: &str) -> Result<tree_sitter::Parser, Error> {
    let language = get_language(name)?;
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&language)
        .map_err(|e| Error::ParserSetup(format!("{e}")))?;
    Ok(parser)
}

/// List all available language names (sorted, deduplicated, includes aliases).
///
/// Returns names of both statically compiled and dynamically loadable languages,
/// plus any configured aliases.
///
/// # Example
///
/// ```no_run
/// use ts_pack_core::available_languages;
///
/// let langs = available_languages();
/// for name in &langs {
///     println!("{}", name);
/// }
/// ```
pub fn available_languages() -> Vec<String> {
    REGISTRY.available_languages()
}

/// Check if a language is available by name or alias.
///
/// Returns `true` if the language can be loaded (statically compiled,
/// dynamically available, or a known alias for one of these).
///
/// # Example
///
/// ```no_run
/// use ts_pack_core::has_language;
///
/// assert!(has_language("python"));
/// assert!(has_language("shell")); // alias for "bash"
/// assert!(!has_language("nonexistent_language"));
/// ```
pub fn has_language(name: &str) -> bool {
    REGISTRY.has_language(name)
}

/// Return the number of available languages.
///
/// Includes statically compiled languages, dynamically loadable languages,
/// and aliases.
///
/// # Example
///
/// ```no_run
/// use ts_pack_core::language_count;
///
/// let count = language_count();
/// println!("{} languages available", count);
/// ```
pub fn language_count() -> usize {
    REGISTRY.language_count()
}

/// Process source code and extract file intelligence using the global registry.
///
/// Parses the source with tree-sitter and extracts metrics, structure, imports,
/// exports, comments, docstrings, symbols, diagnostics, and/or chunks based on
/// the flags set in [`ProcessConfig`].
///
/// # Errors
///
/// Returns an error if the language is not found or parsing fails.
///
/// # Example
///
/// ```no_run
/// use ts_pack_core::{ProcessConfig, process};
///
/// let config = ProcessConfig::new("python").all();
/// let result = process("def hello(): pass", &config).unwrap();
/// println!("Language: {}", result.language);
/// println!("Lines: {}", result.metrics.total_lines);
/// println!("Structures: {}", result.structure.len());
/// ```
pub fn process(source: &str, config: &ProcessConfig) -> Result<ProcessResult, Error> {
    REGISTRY.process(source, config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_available_languages() {
        let langs = available_languages();
        // With zero default parsers, this may be empty unless lang-* features are enabled
        // Verify available_languages doesn't panic; may be empty without lang-* features
        let _ = langs;
    }

    #[test]
    fn test_has_language() {
        let langs = available_languages();
        if !langs.is_empty() {
            assert!(has_language(&langs[0]));
        }
        assert!(!has_language("nonexistent_language_xyz"));
    }

    #[test]
    fn test_get_language_invalid() {
        let result = get_language("nonexistent_language_xyz");
        assert!(result.is_err());
    }

    #[test]
    #[ignore = "loads all 170 dynamic libraries — run with --ignored"]
    fn test_get_language_and_parse() {
        let langs = available_languages();
        for lang_name in &langs {
            let lang = get_language(lang_name.as_str())
                .unwrap_or_else(|e| panic!("Failed to load language '{lang_name}': {e}"));
            let mut parser = tree_sitter::Parser::new();
            parser
                .set_language(&lang)
                .unwrap_or_else(|e| panic!("Failed to set language '{lang_name}': {e}"));
            let tree = parser.parse("x", None);
            assert!(tree.is_some(), "Parser for '{lang_name}' should parse a string");
        }
    }

    #[test]
    fn test_get_parser() {
        let langs = available_languages();
        if let Some(first) = langs.first() {
            let parser = get_parser(first.as_str());
            assert!(parser.is_ok(), "get_parser should succeed for '{first}'");
        }
    }
}
