use std::cell::RefCell;
use std::ops::ControlFlow;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use ahash::AHashMap;

use crate::Error;

thread_local! {
    /// One `tree_sitter::Parser` per language, per thread.
    ///
    /// Unbounded and never evicted, by design: an entry is a parser plus its
    /// grown-to-high-water-mark scratch buffers, and the alternative — dropping
    /// parsers a hot loop is about to ask for again — trades a bounded, per-thread
    /// footprint for repeated reallocation. The ceiling is
    /// `threads x languages-that-thread-parsed`, which for the intended usage
    /// (a worker pool over a handful of languages) is tens of entries.
    ///
    /// It is *not* bounded by the number of languages in the pack unless the
    /// program actually parses them all: the measured 20.4 MB -> 398.4 MB growth
    /// for all 377 grammars is dominated by the loaded shared libraries and
    /// compiled queries, not by this map. Retained `Tree`s are the other large
    /// consumer and belong to the caller — a 23.5 KB source retains ~620 KB of
    /// tree, so holding 100 of them costs ~62 MB no cache can reclaim. ~keep
    static PARSER_CACHE: RefCell<AHashMap<String, tree_sitter::Parser>> = RefCell::new(AHashMap::new());
}

static PARSE_LOCK: Mutex<()> = Mutex::new(());

/// Parse source code with a pre-loaded `Language`, using the thread-local
/// parser cache, cancelling the parse if it exceeds `timeout_ms` milliseconds
/// of wall clock.
///
/// Taking the `Language` avoids a redundant registry lookup when the caller
/// already has one (e.g. from `LanguageRegistry::get_language`).
///
/// `None` means no budget, which is the default everywhere so that existing
/// callers are unaffected.
///
/// Cancellation is granular to tree-sitter's progress-callback interval rather
/// than exact: the callback fires periodically during the parse, so a parse can
/// overrun the budget by up to one interval before it is stopped. ~keep
pub(crate) fn parse_with_language_limited(
    language_name: &str,
    language: &tree_sitter::Language,
    source: &[u8],
    timeout_ms: Option<u64>,
) -> Result<tree_sitter::Tree, Error> {
    // ~keep Some third-party scanners keep process-global state, so parser execution is serialized.
    let _guard = PARSE_LOCK.lock().map_err(|e| Error::LockPoisoned(e.to_string()))?;
    PARSER_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some(parser) = cache.get_mut(language_name) {
            return run_parse(parser, source, timeout_ms);
        }
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(language)
            .map_err(|e| Error::ParserSetup(format!("{e}")))?;
        let tree = run_parse(&mut parser, source, timeout_ms)?;
        cache.insert(language_name.to_string(), parser);
        Ok(tree)
    })
}

/// Run one parse on an already-configured parser, honouring an optional budget.
pub(crate) fn run_parse(
    parser: &mut tree_sitter::Parser,
    source: &[u8],
    timeout_ms: Option<u64>,
) -> Result<tree_sitter::Tree, Error> {
    let len = source.len();
    // ~keep Same reader `tree_sitter::Parser::parse` builds internally. The chunk-callback entry
    // ~keep point is the only one that also takes `ParseOptions`, which carries the cancellation
    // ~keep hook; `set_timeout_micros` no longer exists in tree-sitter 0.26.
    let mut read = |offset: usize, _: tree_sitter::Point| (offset < len).then(|| &source[offset..]).unwrap_or_default();

    let deadline = match timeout_ms {
        Some(budget_ms) => Instant::now().checked_add(Duration::from_millis(budget_ms)),
        None => None,
    };

    let Some(deadline) = deadline else {
        // ~keep No budget configured, or a budget so large its deadline is unrepresentable.
        return parser
            .parse_with_options(&mut read, None, None)
            .ok_or(Error::ParseFailed);
    };

    let mut on_progress = |_: &tree_sitter::ParseState| {
        if Instant::now() < deadline {
            ControlFlow::Continue(())
        } else {
            ControlFlow::Break(())
        }
    };
    let options = tree_sitter::ParseOptions::new().progress_callback(&mut on_progress);

    match parser.parse_with_options(&mut read, None, Some(options)) {
        Some(tree) => Ok(tree),
        None if Instant::now() >= deadline => {
            let budget_ms = timeout_ms.unwrap_or_default();
            tracing::warn!(
                timeout_ms = budget_ms,
                source_bytes = len,
                "parse cancelled: exceeded the configured wall-clock budget"
            );
            Err(Error::ParseTimeout { timeout_ms: budget_ms })
        }
        None => Err(Error::ParseFailed),
    }
}

#[cfg(test)]
pub(crate) fn cached_parser_count_for_tests() -> usize {
    PARSER_CACHE.with(|cache| cache.borrow().len())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn skip_if_no_languages() -> bool {
        crate::available_languages().is_empty()
    }

    fn parse_for_test(language_name: &str, source: &[u8]) -> Result<tree_sitter::Tree, Error> {
        let language = crate::get_language(language_name)?;
        parse_with_language_limited(language_name, &language, source, None)
    }

    #[test]
    fn test_parse_with_language_success() {
        if skip_if_no_languages() {
            return;
        }
        let langs = crate::available_languages();
        let first = &langs[0];
        let tree = parse_for_test(first, b"x");
        assert!(tree.is_ok(), "parse_with_language should succeed for '{first}'");
    }

    #[test]
    fn test_get_language_invalid_language() {
        let result = crate::get_language("nonexistent_xyz");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_with_language_reuses_cache() {
        if skip_if_no_languages() {
            return;
        }
        let langs = crate::available_languages();
        let first = &langs[0];
        let lang = crate::get_language(first).unwrap();
        let _ = parse_with_language_limited(first, &lang, b"x", None).unwrap();
        let after_first = cached_parser_count_for_tests();
        let _ = parse_with_language_limited(first, &lang, b"y", None).unwrap();
        let after_second = cached_parser_count_for_tests();
        assert_eq!(after_first, after_second, "second call should reuse cached parser");
    }

    #[test]
    fn test_different_languages_get_separate_cache_entries() {
        let langs = crate::available_languages();
        if langs.len() < 2 {
            return;
        }
        let before = cached_parser_count_for_tests();
        let _ = parse_for_test(&langs[0], b"x").unwrap();
        let _ = parse_for_test(&langs[1], b"x").unwrap();
        let after = cached_parser_count_for_tests();
        assert!(
            after >= before + 2,
            "different languages should create separate cache entries"
        );
    }
}
