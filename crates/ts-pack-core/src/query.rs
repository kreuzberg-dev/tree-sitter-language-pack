use std::borrow::Cow;
use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::ops::Range;
use std::sync::{Arc, LazyLock, RwLock};
use std::time::Instant;

use crate::Error;
use crate::node::{NodeInfo, node_info_from_node};
use tree_sitter::StreamingIterator;

#[derive(Debug)]
struct CompiledQuery {
    query: tree_sitter::Query,
    capture_names: Vec<Cow<'static, str>>,
}

#[derive(Clone, Debug)]
pub struct PreparedQuery(Arc<CompiledQuery>);

const QUERY_CACHE_SHARDS: usize = 32;

type QueryCacheMap = ahash::AHashMap<(String, String), Arc<CompiledQuery>>;

static QUERY_CACHE: LazyLock<Vec<RwLock<QueryCacheMap>>> = LazyLock::new(|| {
    (0..QUERY_CACHE_SHARDS)
        .map(|_| RwLock::new(QueryCacheMap::new()))
        .collect()
});

thread_local! {
    static LOCAL_QUERY_CACHE: RefCell<QueryCacheMap> = RefCell::new(QueryCacheMap::new());
}

/// A single match from a tree-sitter query, with captured nodes.
#[derive(Debug, Clone)]
pub struct QueryMatch {
    /// The pattern index that matched (position in the query string).
    pub pattern_index: usize,
    /// Captures: list of (capture_name, node_info) pairs.
    pub captures: Vec<(Cow<'static, str>, NodeInfo)>,
}

/// Profiling metadata for a single tree-sitter query execution.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct QueryProfile {
    /// Time spent resolving the compiled query from cache / compiling on miss.
    pub lookup_secs: f64,
    /// Number of matches returned by the cursor.
    pub match_count: usize,
    /// Whether tree-sitter reported exceeding its in-flight match limit.
    pub exceeded_match_limit: bool,
    /// Whether a byte range restriction was applied.
    pub used_byte_range: bool,
    /// Elapsed execution time in seconds.
    pub elapsed_secs: f64,
}

/// Execute a tree-sitter query pattern against a parsed tree.
///
/// The `query_source` is an S-expression pattern like:
/// ```text
/// (function_definition name: (identifier) @name)
/// ```
///
/// Returns all matches with their captured nodes.
///
/// # Arguments
///
/// * `tree` - The parsed syntax tree to query.
/// * `language` - Language name (used to compile the query pattern).
/// * `query_source` - The tree-sitter query pattern string.
/// * `source` - The original source code bytes (needed for capture resolution).
///
/// # Examples
///
/// ```no_run
/// let tree = tree_sitter_language_pack::parse::parse_string("python", b"def hello(): pass").unwrap();
/// let matches = tree_sitter_language_pack::query::run_query(
///     &tree,
///     "python",
///     "(function_definition name: (identifier) @fn_name)",
///     b"def hello(): pass",
/// ).unwrap();
/// assert!(!matches.is_empty());
/// ```
pub fn run_query(
    tree: &tree_sitter::Tree,
    language: &str,
    query_source: &str,
    source: &[u8],
) -> Result<Vec<QueryMatch>, Error> {
    let query = compiled_query(language, query_source)?;
    collect_query_matches(tree, &query, source, None)
}

pub fn prepare_query(language: &str, query_source: &str) -> Result<PreparedQuery, Error> {
    compiled_query(language, query_source).map(PreparedQuery)
}

pub fn run_prepared_query(
    tree: &tree_sitter::Tree,
    prepared: &PreparedQuery,
    source: &[u8],
) -> Result<Vec<QueryMatch>, Error> {
    collect_query_matches(tree, &prepared.0, source, None)
}

pub fn run_prepared_query_in_byte_range(
    tree: &tree_sitter::Tree,
    prepared: &PreparedQuery,
    source: &[u8],
    byte_range: Range<usize>,
) -> Result<Vec<QueryMatch>, Error> {
    collect_query_matches(tree, &prepared.0, source, Some(byte_range))
}

/// Execute a query and return both matches and profiling metadata.
pub fn run_query_profiled(
    tree: &tree_sitter::Tree,
    language: &str,
    query_source: &str,
    source: &[u8],
) -> Result<(Vec<QueryMatch>, QueryProfile), Error> {
    let lookup_started = Instant::now();
    let query = compiled_query(language, query_source)?;
    let (matches, mut profile) = collect_query_matches_profiled(tree, &query, source, None)?;
    profile.lookup_secs = (lookup_started.elapsed().as_secs_f64() - profile.elapsed_secs).max(0.0);
    Ok((matches, profile))
}

pub fn run_prepared_query_profiled(
    tree: &tree_sitter::Tree,
    prepared: &PreparedQuery,
    source: &[u8],
) -> Result<(Vec<QueryMatch>, QueryProfile), Error> {
    collect_query_matches_profiled(tree, &prepared.0, source, None)
}

pub fn run_prepared_query_in_byte_range_profiled(
    tree: &tree_sitter::Tree,
    prepared: &PreparedQuery,
    source: &[u8],
    byte_range: Range<usize>,
) -> Result<(Vec<QueryMatch>, QueryProfile), Error> {
    collect_query_matches_profiled(tree, &prepared.0, source, Some(byte_range))
}

/// Validate that a query compiles for the given language.
pub fn query_compiles(language: &str, query_source: &str) -> bool {
    compiled_query(language, query_source).is_ok()
}

/// Execute a tree-sitter query pattern against a parsed tree within a byte range.
///
/// Only matches fully contained within `byte_range` are returned.
pub fn run_query_in_byte_range(
    tree: &tree_sitter::Tree,
    language: &str,
    query_source: &str,
    source: &[u8],
    byte_range: Range<usize>,
) -> Result<Vec<QueryMatch>, Error> {
    let query = compiled_query(language, query_source)?;
    collect_query_matches(tree, &query, source, Some(byte_range))
}

/// Execute a query within a byte range and return profiling metadata.
pub fn run_query_in_byte_range_profiled(
    tree: &tree_sitter::Tree,
    language: &str,
    query_source: &str,
    source: &[u8],
    byte_range: Range<usize>,
) -> Result<(Vec<QueryMatch>, QueryProfile), Error> {
    let lookup_started = Instant::now();
    let query = compiled_query(language, query_source)?;
    let (matches, mut profile) = collect_query_matches_profiled(tree, &query, source, Some(byte_range))?;
    profile.lookup_secs = (lookup_started.elapsed().as_secs_f64() - profile.elapsed_secs).max(0.0);
    Ok((matches, profile))
}

fn collect_query_matches(
    tree: &tree_sitter::Tree,
    query: &CompiledQuery,
    source: &[u8],
    byte_range: Option<Range<usize>>,
) -> Result<Vec<QueryMatch>, Error> {
    let (results, _) = collect_query_matches_profiled(tree, query, source, byte_range)?;
    Ok(results)
}

fn collect_query_matches_profiled(
    tree: &tree_sitter::Tree,
    query: &CompiledQuery,
    source: &[u8],
    byte_range: Option<Range<usize>>,
) -> Result<(Vec<QueryMatch>, QueryProfile), Error> {
    let started = Instant::now();
    let mut cursor = tree_sitter::QueryCursor::new();
    let used_byte_range = byte_range.is_some();
    if let Some(range) = byte_range {
        cursor.set_containing_byte_range(range);
    }
    let mut matches = cursor.matches(&query.query, tree.root_node(), source);

    // Tree-sitter 0.26+ evaluates standard text predicates (`#eq?`, `#not-eq?`,
    // `#match?`, `#not-match?`, `#any-of?`, `#not-any-of?`) internally via
    // `satisfies_text_predicates()` during `QueryCursor::matches()` iteration.
    // The `general_predicates()` method only returns predicates with operators
    // that tree-sitter does NOT recognize (i.e., custom predicates). Since we
    // don't define any custom predicates, no additional filtering is needed.
    let mut results = Vec::new();
    while let Some(m) = matches.next() {
        let captures = m
            .captures
            .iter()
            .map(|c| {
                let name = query.capture_names[c.index as usize].clone();
                let info = node_info_from_node(c.node);
                (name, info)
            })
            .collect();
        results.push(QueryMatch {
            pattern_index: m.pattern_index,
            captures,
        });
    }
    let profile = QueryProfile {
        lookup_secs: 0.0,
        match_count: results.len(),
        exceeded_match_limit: cursor.did_exceed_match_limit(),
        used_byte_range,
        elapsed_secs: started.elapsed().as_secs_f64(),
    };
    Ok((results, profile))
}

fn compiled_query(language: &str, query_source: &str) -> Result<Arc<CompiledQuery>, Error> {
    let key = (language.to_string(), query_source.to_string());
    if let Some(query) = LOCAL_QUERY_CACHE.with(|cache| cache.borrow().get(&key).cloned()) {
        return Ok(query);
    }
    let shard_idx = query_cache_shard(&key);
    if let Some(query) = QUERY_CACHE[shard_idx]
        .read()
        .ok()
        .and_then(|cache| cache.get(&key).cloned())
    {
        LOCAL_QUERY_CACHE.with(|cache| {
            cache.borrow_mut().insert(key.clone(), Arc::clone(&query));
        });
        return Ok(query);
    }

    let lang = crate::get_language(language)?;
    let query = tree_sitter::Query::new(&lang, query_source).map_err(|e| Error::QueryError(format!("{e}")))?;
    let capture_names = query
        .capture_names()
        .iter()
        .map(|s| Cow::Owned(s.to_string()))
        .collect();
    let query = Arc::new(CompiledQuery { query, capture_names });
    LOCAL_QUERY_CACHE.with(|cache| {
        cache.borrow_mut().insert(key.clone(), Arc::clone(&query));
    });
    if let Ok(mut cache) = QUERY_CACHE[shard_idx].write() {
        Ok(cache.entry(key).or_insert_with(|| Arc::clone(&query)).clone())
    } else {
        Ok(query)
    }
}

fn query_cache_shard(key: &(String, String)) -> usize {
    let mut hasher = ahash::AHasher::default();
    key.hash(&mut hasher);
    (hasher.finish() as usize) % QUERY_CACHE_SHARDS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_query_invalid_language() {
        // Create a dummy tree from any available language
        let langs = crate::available_languages();
        if langs.is_empty() {
            return;
        }
        let tree = crate::parse::parse_string(&langs[0], b"x").unwrap();
        let result = run_query(&tree, "nonexistent_xyz", "(identifier) @id", b"x");
        assert!(result.is_err());
    }

    #[test]
    fn test_run_query_invalid_pattern() {
        let langs = crate::available_languages();
        if langs.is_empty() {
            return;
        }
        let first = &langs[0];
        let tree = crate::parse::parse_string(first, b"x").unwrap();
        let result = run_query(&tree, first, "((((invalid syntax", b"x");
        assert!(result.is_err());
    }

    #[test]
    fn test_run_query_no_matches() {
        let langs = crate::available_languages();
        if langs.is_empty() {
            return;
        }
        let first = &langs[0];
        let tree = crate::parse::parse_string(first, b"x").unwrap();
        // Query for a node type that is unlikely to exist for a single "x"
        let result = run_query(&tree, first, "(function_definition) @fn", b"x");
        // This might error if the grammar doesn't have function_definition,
        // or return empty matches. Either is acceptable.
        if let Ok(matches) = result {
            assert!(matches.is_empty());
        }
        // Query compilation error is fine for some grammars
    }

    #[test]
    fn test_compiled_query_reused() {
        let langs = crate::available_languages();
        if langs.is_empty() {
            return;
        }
        let first = &langs[0];
        let query_src = "(identifier) @id";
        let q1 = compiled_query(first, query_src).unwrap();
        let q2 = compiled_query(first, query_src).unwrap();
        assert!(Arc::ptr_eq(&q1, &q2));
    }

    #[test]
    fn test_run_query_in_byte_range_limits_results() {
        if !crate::has_language("python") {
            return;
        }
        let source = b"def first():\n    pass\n\ndef second():\n    pass\n";
        let tree = crate::parse::parse_string("python", source).unwrap();
        let all = run_query(&tree, "python", "(function_definition name: (identifier) @fn)", source).unwrap();
        assert_eq!(all.len(), 2);

        let second_start = std::str::from_utf8(source).unwrap().find("def second").unwrap();
        let ranged = run_query_in_byte_range(
            &tree,
            "python",
            "(function_definition name: (identifier) @fn)",
            source,
            second_start..source.len(),
        )
        .unwrap();
        assert_eq!(ranged.len(), 1);
        let fn_name = ranged[0]
            .captures
            .iter()
            .find(|(cap, _)| cap.as_ref() == "fn")
            .and_then(|(_, info)| crate::extract_text(source, info).ok())
            .unwrap();
        assert_eq!(fn_name, "second");
    }

    #[test]
    fn test_run_query_profiled_reports_stats() {
        if !crate::has_language("python") {
            return;
        }
        let source = b"def first():\n    pass\n\ndef second():\n    pass\n";
        let tree = crate::parse::parse_string("python", source).unwrap();
        let (matches, profile) =
            run_query_profiled(&tree, "python", "(function_definition name: (identifier) @fn)", source).unwrap();
        assert_eq!(matches.len(), 2);
        assert_eq!(profile.match_count, 2);
        assert!(!profile.used_byte_range);
        assert!(profile.elapsed_secs >= 0.0);
    }

    #[test]
    fn test_run_prepared_query_profiled_reports_stats() {
        if !crate::has_language("python") {
            return;
        }
        let source = b"def first():\n    pass\n\ndef second():\n    pass\n";
        let tree = crate::parse::parse_string("python", source).unwrap();
        let prepared = prepare_query("python", "(function_definition name: (identifier) @fn)").unwrap();
        let (matches, profile) = run_prepared_query_profiled(&tree, &prepared, source).unwrap();
        assert_eq!(matches.len(), 2);
        assert_eq!(profile.match_count, 2);
        assert_eq!(profile.lookup_secs, 0.0);
        assert!(profile.elapsed_secs >= 0.0);
    }

    #[test]
    fn test_run_query_in_byte_range_profiled_reports_range_usage() {
        if !crate::has_language("python") {
            return;
        }
        let source = b"def first():\n    pass\n\ndef second():\n    pass\n";
        let tree = crate::parse::parse_string("python", source).unwrap();
        let second_start = std::str::from_utf8(source).unwrap().find("def second").unwrap();
        let (matches, profile) = run_query_in_byte_range_profiled(
            &tree,
            "python",
            "(function_definition name: (identifier) @fn)",
            source,
            second_start..source.len(),
        )
        .unwrap();
        assert_eq!(matches.len(), 1);
        assert!(profile.used_byte_range);
    }
}
