pub mod error;
pub mod registry;

#[cfg(feature = "config")]
pub mod config;
#[cfg(feature = "config")]
pub mod definitions;

pub use error::Error;
pub use registry::LanguageRegistry;
pub use tree_sitter::Language;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_available_languages_not_empty() {
        let langs = available_languages();
        assert!(!langs.is_empty(), "Should have at least one language compiled");
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
