use std::borrow::Cow;
use std::cell::RefCell;
use std::sync::{Arc, LazyLock, RwLock};

use crate::Error;
use crate::node::{NodeInfo, node_info_from_node};
use tree_sitter::StreamingIterator;

#[derive(Debug)]
struct CompiledQuery {
    query: tree_sitter::Query,
    capture_names: Vec<Cow<'static, str>>,
}

type QueryCacheMap = ahash::AHashMap<(String, String), Arc<CompiledQuery>>;

static QUERY_CACHE: LazyLock<RwLock<QueryCacheMap>> = LazyLock::new(|| RwLock::new(QueryCacheMap::new()));

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

    let mut cursor = tree_sitter::QueryCursor::new();
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
    Ok(results)
}

fn compiled_query(language: &str, query_source: &str) -> Result<Arc<CompiledQuery>, Error> {
    let key = (language.to_string(), query_source.to_string());
    if let Some(query) = LOCAL_QUERY_CACHE.with(|cache| cache.borrow().get(&key).cloned()) {
        return Ok(query);
    }
    if let Some(query) = QUERY_CACHE.read().ok().and_then(|cache| cache.get(&key).cloned()) {
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
    if let Ok(mut cache) = QUERY_CACHE.write() {
        Ok(cache.entry(key).or_insert_with(|| Arc::clone(&query)).clone())
    } else {
        Ok(query)
    }
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
}
