use crate::Error;
use crate::node::{NodeInfo, node_info_from_node};
use tree_sitter::StreamingIterator;

/// A single match from a tree-sitter query, with captured nodes.
#[derive(Debug, Clone)]
pub struct QueryMatch {
    /// The pattern index that matched (position in the query string).
    pub pattern_index: usize,
    /// Captures: list of (capture_name, node_info) pairs.
    pub captures: Vec<(String, NodeInfo)>,
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
/// let tree = ts_pack_core::parse::parse_string("python", b"def hello(): pass").unwrap();
/// let matches = ts_pack_core::query::run_query(
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
    let lang = crate::get_language(language)?;
    let query = tree_sitter::Query::new(&lang, query_source).map_err(|e| Error::QueryError(format!("{e}")))?;
    let capture_names: Vec<String> = query.capture_names().iter().map(|s| s.to_string()).collect();

    let mut cursor = tree_sitter::QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), source);

    let mut results = Vec::new();
    while let Some(m) = matches.next() {
        let captures = m
            .captures
            .iter()
            .map(|c| {
                let name = capture_names[c.index as usize].clone();
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
}
