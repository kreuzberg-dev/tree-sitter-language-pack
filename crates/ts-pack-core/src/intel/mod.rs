//! Content intelligence and code chunking using tree-sitter.
//!
//! This module provides rich AST metadata extraction and intelligent code chunking.
//! It analyzes source code to extract structure, imports, exports, comments,
//! docstrings, symbols, and diagnostics.

pub mod chunking;
pub mod intelligence;
pub mod types;

pub use types::*;

/// Process source code: parse once, extract intelligence and return it.
pub fn process(
    source: &str,
    language: &str,
    registry: &crate::LanguageRegistry,
) -> Result<FileIntelligence, crate::Error> {
    let (_lang, tree) = parse_source(source, language, registry)?;
    Ok(intelligence::extract_intelligence(source, language, &tree))
}

/// Process and chunk source code in a single pass.
///
/// Parses once and extracts both file-level intelligence and per-chunk metadata.
pub fn process_and_chunk(
    source: &str,
    language: &str,
    max_chunk_size: usize,
    registry: &crate::LanguageRegistry,
) -> Result<(FileIntelligence, Vec<IntelligentChunk>), crate::Error> {
    let (lang, tree) = parse_source(source, language, registry)?;
    let intel = intelligence::extract_intelligence(source, language, &tree);
    let chunks = chunking::chunk_source(source, language, max_chunk_size, &lang, &tree);
    Ok((intel, chunks))
}

/// Parse source code and return the tree-sitter language and tree.
fn parse_source(
    source: &str,
    language: &str,
    registry: &crate::LanguageRegistry,
) -> Result<(tree_sitter::Language, tree_sitter::Tree), crate::Error> {
    let lang = registry.get_language(language)?;
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&lang)
        .map_err(|e| crate::Error::ParserSetup(e.to_string()))?;
    let tree = parser.parse(source, None).ok_or(crate::Error::ParseFailed)?;
    Ok((lang, tree))
}

#[cfg(test)]
mod tests {
    use crate::LanguageRegistry;

    fn first_lang(registry: &LanguageRegistry) -> Option<String> {
        let langs = registry.available_languages();
        langs.into_iter().next()
    }

    #[test]
    fn test_process_returns_intelligence() {
        let registry = LanguageRegistry::new();
        let Some(lang) = first_lang(&registry) else { return };
        let source = "x";
        let result = super::process(source, &lang, &registry);
        assert!(result.is_ok(), "process should succeed for available language");
        let intel = result.unwrap();
        assert_eq!(intel.language, lang);
        assert!(intel.metrics.total_lines >= 1);
        assert!(intel.metrics.node_count > 0);
    }

    #[test]
    fn test_process_and_chunk_returns_both() {
        let registry = LanguageRegistry::new();
        let Some(lang) = first_lang(&registry) else { return };
        let source = "x";
        let result = super::process_and_chunk(source, &lang, 1000, &registry);
        assert!(result.is_ok());
        let (intel, chunks) = result.unwrap();
        assert_eq!(intel.language, lang);
        assert!(!chunks.is_empty(), "should have at least one chunk");
        assert_eq!(chunks[0].metadata.language, lang);
    }

    #[test]
    fn test_process_invalid_language() {
        let registry = LanguageRegistry::new();
        let result = super::process("x", "nonexistent_lang_xyz", &registry);
        assert!(result.is_err(), "should fail for nonexistent language");
    }

    #[test]
    fn test_process_empty_source() {
        let registry = LanguageRegistry::new();
        let Some(lang) = first_lang(&registry) else { return };
        let result = super::process("", &lang, &registry);
        assert!(result.is_ok(), "empty source should parse without error");
        let intel = result.unwrap();
        assert_eq!(intel.metrics.total_bytes, 0);
    }

    #[test]
    fn test_process_and_chunk_small_max_size() {
        let registry = LanguageRegistry::new();
        if !registry.has_language("python") {
            return;
        }
        let source = "def foo():\n    pass\ndef bar():\n    pass\n";
        let result = super::process_and_chunk(source, "python", 20, &registry);
        assert!(result.is_ok());
        let (intel, chunks) = result.unwrap();
        assert!(
            chunks.len() >= 2,
            "small max_chunk_size should split into multiple chunks"
        );
        assert_eq!(intel.language, "python");
    }
}
