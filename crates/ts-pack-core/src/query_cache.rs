//! Process-wide cache of compiled tree-sitter queries.
//!
//! Compiling a [`tree_sitter::Query`] from a bundled `.scm` source is relatively
//! expensive, and the result is immutable and `Send + Sync`, so it is cached once
//! per `(language, kind)` and shared via an `Arc` for the lifetime of the process.
//!
//! The cache is Rust-only: `Arc<Query>` is an opaque handle that cannot cross the C
//! FFI boundary, so [`get_query`] and [`QueryKind`] are excluded from the generated
//! bindings (`alef(skip)`). Non-Rust callers use the raw query-string accessors
//! ([`crate::get_tags_query`] et al.) and compile with their own tree-sitter runtime.
//!
//! # Memory ceiling
//!
//! The cache is deliberately unbounded and never evicts. An entry is only created by
//! an explicit [`get_query`] call, so the ceiling is set by what the process asks for,
//! and eviction would be a bad trade here: compiling one query costs milliseconds to
//! tens of milliseconds (measured: `go`/highlights 1.2 ms, `rust`/highlights 11.5 ms,
//! `ruby`/highlights 19.6 ms, `swift`/highlights 85 ms — swift's six queries are
//! ~263 ms of one-time latency), while an entry costs a few hundred KB, and any
//! capacity bound small enough to matter would re-pay that compile on the next miss.
//! Callers also hold `Arc<Query>` clones, so eviction frees nothing until they drop.
//!
//! Measured worst case: loading all 377 grammars with all six query kinds takes the
//! process from a 20.4 MB baseline to 398.4 MB, i.e. ~1 MB per grammar including the
//! grammar's own shared library. A process that touches ten languages pays ~10 MB. A
//! caller that needs a hard bound should restrict which languages it asks for; there
//! is no supported way to unload a grammar, because a `Language` borrows code from a
//! `dlopen`ed library that must outlive every tree parsed with it. ~keep

use std::sync::{Arc, LazyLock, RwLock};

use ahash::AHashMap;
use tree_sitter::Query;

use crate::error::Error;

/// Which bundled `.scm` query to compile and cache for a language.
#[cfg_attr(alef, alef(skip))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QueryKind {
    /// Syntax-highlighting queries (`highlights.scm`).
    Highlights,
    /// Language-injection queries (`injections.scm`).
    Injections,
    /// Local-scope / variable-resolution queries (`locals.scm`).
    Locals,
    /// Symbol/definition tag queries (`tags.scm`).
    Tags,
    /// Auto-indentation queries (`indents.scm`).
    Indents,
    /// Code-folding queries (`folds.scm`).
    Folds,
}

/// Number of variants of [`QueryKind`]; the cache is one map per kind.
const QUERY_KIND_COUNT: usize = 6;

/// Index of a kind in [`QUERY_CACHE`]. A free function, not an inherent method, so
/// nothing is added to the public surface of the `alef(skip)`ped enum. ~keep
const fn cache_index(kind: QueryKind) -> usize {
    match kind {
        QueryKind::Highlights => 0,
        QueryKind::Injections => 1,
        QueryKind::Locals => 2,
        QueryKind::Tags => 3,
        QueryKind::Indents => 4,
        QueryKind::Folds => 5,
    }
}

type QueryMap = AHashMap<Box<str>, Option<Arc<Query>>>;

/// One map per [`QueryKind`], so a lookup keys on the language alone.
///
/// A tuple key `(String, QueryKind)` has no `Borrow<(&str, QueryKind)>` impl, so every
/// lookup had to allocate the language name: 16.2 ns of a 19.7 ns cache hit. Splitting
/// the kind out into the array index makes the hot path a borrowed `&str` lookup with
/// no allocation, and gives each kind its own lock. ~keep
#[allow(clippy::type_complexity)]
static QUERY_CACHE: LazyLock<[RwLock<QueryMap>; QUERY_KIND_COUNT]> =
    LazyLock::new(|| std::array::from_fn(|_| RwLock::new(AHashMap::new())));

/// The raw `.scm` source string for a language/kind, or `None` if not bundled.
fn source_for(language: &str, kind: QueryKind) -> Option<&'static str> {
    match kind {
        QueryKind::Highlights => crate::queries::get_highlights_query(language),
        QueryKind::Injections => crate::queries::get_injections_query(language),
        QueryKind::Locals => crate::queries::get_locals_query(language),
        QueryKind::Tags => crate::queries::get_tags_query(language),
        QueryKind::Indents => crate::queries::get_indents_query(language),
        QueryKind::Folds => crate::queries::get_folds_query(language),
    }
}

/// Get a compiled, process-cached tree-sitter query for `language` / `kind`.
///
/// Returns `Ok(None)` when no `.scm` source is bundled for that language and kind —
/// the negative result is cached, so a missing overlay is checked exactly once. The
/// compiled [`Query`] is shared via an `Arc` and lives for the process lifetime.
///
/// Language aliases are resolved (e.g. `"shell"` → `"bash"`), consistent with
/// [`crate::get_language`].
///
/// # Errors
///
/// Returns [`Error::QueryError`] if the bundled `.scm` source fails to compile, or
/// [`Error::LanguageNotFound`] if the language cannot be loaded.
///
/// # Example
///
/// ```
/// use tree_sitter_language_pack::{get_query, QueryKind};
///
/// // No query is bundled for an unknown language → Ok(None).
/// assert!(get_query("nonexistent_lang", QueryKind::Tags)?.is_none());
/// # Ok::<(), tree_sitter_language_pack::Error>(())
/// ```
#[cfg_attr(alef, alef(skip))]
pub fn get_query(language: &str, kind: QueryKind) -> Result<Option<Arc<Query>>, Error> {
    let lang = crate::registry::resolve_alias(language);
    let cache = &QUERY_CACHE[cache_index(kind)];

    // ~keep Read fast path avoids taking the write lock on cached queries.
    {
        let map = cache.read().map_err(|e| Error::LockPoisoned(e.to_string()))?;
        if let Some(entry) = map.get(lang) {
            return Ok(entry.clone());
        }
    }

    // ~keep Compile outside the write lock to keep the critical section small.
    let compiled: Option<Arc<Query>> = match source_for(lang, kind) {
        None => None,
        Some(src) => {
            let language = crate::get_language(lang)?;
            let query = Query::new(&language, src)
                .map_err(|e| Error::QueryError(format!("failed to compile {kind:?} query for '{lang}': {e}")))?;
            Some(Arc::new(query))
        }
    };

    // ~keep Double-check on write: a concurrent compile wins and we drop ours.
    let mut map = cache.write().map_err(|e| Error::LockPoisoned(e.to_string()))?;
    let entry = map.entry(Box::from(lang)).or_insert(compiled);
    Ok(entry.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL_KINDS: [QueryKind; 6] = [
        QueryKind::Highlights,
        QueryKind::Injections,
        QueryKind::Locals,
        QueryKind::Tags,
        QueryKind::Indents,
        QueryKind::Folds,
    ];

    #[test]
    fn every_kind_owns_a_distinct_cache_slot() {
        assert_eq!(
            ALL_KINDS.len(),
            QUERY_KIND_COUNT,
            "ALL_KINDS must cover every QueryKind"
        );
        let mut claimed = [false; QUERY_KIND_COUNT];
        for kind in ALL_KINDS {
            let index = cache_index(kind);
            assert!(
                index < QUERY_KIND_COUNT,
                "cache index {index} out of range for {kind:?}"
            );
            assert!(!claimed[index], "two kinds share cache slot {index} ({kind:?})");
            claimed[index] = true;
        }
        assert!(claimed.iter().all(|&slot| slot), "every cache slot must be claimed");
    }

    #[test]
    fn missing_query_returns_none_and_is_negatively_cached() {
        // ~keep Unknown language has no bundled source; negative cache avoids loading a grammar.
        for _ in 0..2 {
            for kind in ALL_KINDS {
                assert!(get_query("definitely_not_a_language", kind).unwrap().is_none());
            }
        }
    }
}
