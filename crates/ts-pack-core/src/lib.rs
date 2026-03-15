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

/// Get a tree-sitter Language by name using the global registry.
pub fn get_language(name: &str) -> Result<Language, Error> {
    REGISTRY.get_language(name)
}

/// Get a tree-sitter Parser pre-configured for the given language.
pub fn get_parser(name: &str) -> Result<tree_sitter::Parser, Error> {
    let language = get_language(name)?;
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&language)
        .map_err(|e| Error::ParserSetup(format!("{e}")))?;
    Ok(parser)
}

/// List all available language names.
pub fn available_languages() -> Vec<String> {
    REGISTRY.available_languages()
}

/// Check if a language is available.
pub fn has_language(name: &str) -> bool {
    REGISTRY.has_language(name)
}

/// Return the number of available languages without allocating.
pub fn language_count() -> usize {
    REGISTRY.language_count()
}

/// Process source code and extract file intelligence using the global registry.
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
