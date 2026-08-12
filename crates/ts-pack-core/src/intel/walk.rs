//! Depth-bounded, allocation-light AST traversal shared by every intel extractor.
//!
//! Native recursion over a tree-sitter tree is unbounded: a file of nested
//! brackets produces one stack frame per nesting level, and a Rust stack
//! overflow is a SIGSEGV/abort that `catch_unwind` at the FFI boundary cannot
//! contain. Every walker in this module family therefore goes through
//! [`walk_bounded`], which keeps its own explicit cursor stack (constant native
//! stack usage) and refuses to descend past [`MAX_TREE_DEPTH`].
//!
//! The traversal shape mirrors [`crate::text_splitter`]'s `collect_node_ranges`:
//! a single [`tree_sitter::TreeCursor`] driven with
//! `goto_first_child`/`goto_next_sibling`/`goto_parent`, visiting nodes in
//! pre-order — the same order the previous recursive walkers used, so output
//! ordering is unchanged.

use tree_sitter::Node;

/// Maximum AST depth any intel traversal descends before truncating.
///
/// Sized against the smallest stack the library actually runs on: a 2 MiB
/// thread stack (tokio `spawn_blocking`, and Node/Python/JVM worker threads),
/// where the old recursive walkers aborted at roughly 3,000 levels — about
/// 700 bytes of stack per level. The recursion that survives this module
/// (`data_extraction`'s per-format tree builders) is bounded by this constant,
/// so its worst case is about 512 * 700 B = 360 KiB, roughly a sixth of a
/// 2 MiB stack and a ~6x margin below the measured abort point. It is also
/// several times deeper than the deepest AST real source code produces, so
/// non-pathological input is never truncated.
pub(crate) const MAX_TREE_DEPTH: usize = 512;

/// Whether [`walk_bounded`] should descend into the visited node's children.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Descend {
    /// Visit the node's children next.
    Children,
    /// Skip the node's subtree entirely; it is not counted as truncated.
    Skip,
}

/// Walk `root` in pre-order, calling `visit` with each node and its depth
/// (`root` is depth 0), never descending deeper than [`MAX_TREE_DEPTH`].
///
/// Returns the number of nodes that were dropped because the depth limit was
/// reached. A non-zero return means the caller's results are partial and the
/// caller is expected to report it (see [`warn_if_truncated`]).
pub(crate) fn walk_bounded<'tree, F>(root: &Node<'tree>, mut visit: F) -> usize
where
    F: FnMut(&Node<'tree>, usize) -> Descend,
{
    let mut truncated = 0usize;
    let mut cursor = root.walk();
    let mut depth = 0usize;

    loop {
        let node = cursor.node();
        let wants_children = visit(&node, depth) == Descend::Children;

        if wants_children && depth >= MAX_TREE_DEPTH {
            // ~keep `descendant_count` includes the node itself; only its subtree is dropped.
            truncated += node.descendant_count().saturating_sub(1);
        } else if wants_children && cursor.goto_first_child() {
            depth += 1;
            continue;
        }

        loop {
            if cursor.goto_next_sibling() {
                break;
            }
            if depth == 0 || !cursor.goto_parent() {
                return truncated;
            }
            depth -= 1;
        }
    }
}

/// Emit the standard WARN for a truncated walk. Truncation is degraded-but-
/// continuing behaviour: callers return partial results rather than erroring.
pub(crate) fn warn_if_truncated(truncated: usize, operation: &'static str, language: &str) {
    if truncated == 0 {
        return;
    }
    tracing::warn!(
        target: "ts_pack::intel",
        operation,
        language,
        max_depth = MAX_TREE_DEPTH,
        skipped_nodes = truncated,
        "AST deeper than the traversal depth limit; results are truncated"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    // ~keep A missing grammar reports `SKIPPED` on stderr; see `intel::test_support`.
    fn parse_or_skip(source: &str, language: &str) -> Option<tree_sitter::Tree> {
        crate::intel::test_support::parse_or_skip(source, language)
    }

    #[test]
    fn should_visit_every_node_in_preorder_for_a_shallow_tree() {
        let Some(tree) = parse_or_skip("fn main() { let x = 1; }", "rust") else {
            return;
        };
        let root = tree.root_node();

        let mut visited = Vec::new();
        let truncated = walk_bounded(&root, |node, depth| {
            visited.push((node.id(), depth));
            Descend::Children
        });

        assert_eq!(truncated, 0, "a shallow tree must not truncate");
        assert_eq!(
            visited.len(),
            root.descendant_count(),
            "walk must visit exactly every node of the tree"
        );
        assert_eq!(visited[0], (root.id(), 0), "root is visited first at depth 0");
    }

    #[test]
    fn should_not_descend_when_visitor_returns_skip() {
        let Some(tree) = parse_or_skip("fn main() { let x = 1; }", "rust") else {
            return;
        };
        let root = tree.root_node();

        let mut count = 0usize;
        let truncated = walk_bounded(&root, |_, _| {
            count += 1;
            Descend::Skip
        });

        assert_eq!(count, 1, "skipping the root must visit only the root");
        assert_eq!(truncated, 0, "an explicit skip is not a truncation");
    }

    #[test]
    fn should_truncate_and_report_skipped_nodes_when_deeper_than_limit() {
        // ~keep Brackets must balance: an unclosed run parses to one flat ERROR node, not a deep tree.
        let nesting = MAX_TREE_DEPTH + 200;
        let source = format!("{}1{}", "[".repeat(nesting), "]".repeat(nesting));
        let Some(tree) = parse_or_skip(&source, "python") else {
            return;
        };
        let root = tree.root_node();

        let mut max_seen = 0usize;
        let truncated = walk_bounded(&root, |_, depth| {
            max_seen = max_seen.max(depth);
            Descend::Children
        });

        assert_eq!(max_seen, MAX_TREE_DEPTH, "walk must stop exactly at the depth limit");
        assert!(truncated > 0, "over-deep input must report skipped nodes");
    }
}
