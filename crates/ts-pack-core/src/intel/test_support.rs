//! Grammar-gated test helpers shared by the `intel` test modules.
//!
//! A default build of this crate links **zero** grammars, so a test that
//! silently `return`s when its grammar is missing passes while asserting
//! nothing. Every helper here reports the skip on stderr instead, so the skips
//! in a run are visible and countable:
//!
//! ```text
//! cargo test -p ts-pack-core 2>&1 | grep -c '^SKIPPED'
//! ```

// ~keep Test-only diagnostics: a test binary installs no tracing subscriber, so stderr is the surface.
#![allow(clippy::print_stderr)]

/// Report that the running test asserted nothing, and why.
///
/// ~keep The harness names each test thread after the test, so parallel runs
/// ~keep identify the skipped test; a single-threaded run reports `main`.
fn report_skip(reason: &str) {
    let thread = std::thread::current();
    let test = thread.name().unwrap_or("<unnamed test>");
    eprintln!("SKIPPED {test}: {reason}");
}

/// Load `name` from the registry, reporting a skip when the grammar is absent.
pub(super) fn language_or_skip(name: &str) -> Option<tree_sitter::Language> {
    let registry = crate::LanguageRegistry::new();
    match registry.get_language(name) {
        Ok(language) => Some(language),
        Err(error) => {
            report_skip(&format!("grammar '{name}' is not available: {error}"));
            None
        }
    }
}

/// Parse `source` with `language`, reporting a skip when the grammar is absent.
pub(super) fn parse_or_skip(source: &str, language: &str) -> Option<tree_sitter::Tree> {
    let (_, tree) = parse_with_language_or_skip(source, language)?;
    Some(tree)
}

/// Like [`parse_or_skip`], but also returns the language for callers that need
/// it (chunking takes one).
pub(super) fn parse_with_language_or_skip(
    source: &str,
    language: &str,
) -> Option<(tree_sitter::Language, tree_sitter::Tree)> {
    let loaded = language_or_skip(language)?;
    let mut parser = tree_sitter::Parser::new();
    if let Err(error) = parser.set_language(&loaded) {
        report_skip(&format!("grammar '{language}' has an incompatible ABI: {error}"));
        return None;
    }
    match parser.parse(source, None) {
        Some(tree) => Some((loaded, tree)),
        None => {
            report_skip(&format!("grammar '{language}' returned no tree"));
            None
        }
    }
}
