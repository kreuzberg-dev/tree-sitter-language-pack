---
title: Extraction queries
description: "Custom extraction queries are not part of the public API — use process(), bundled query sources, or manual AST traversal."
---

## Extraction queries

Custom query execution helpers are not exported by the Rust crate or the generated language bindings.

Use [`process()`](/guides/intelligence/) for supported code intelligence fields such as structure, imports, exports, comments, docstrings, symbols, diagnostics, metrics, and chunks. The implementation extracts these fields with manual AST traversal in the Rust core.

Bundled query helper functions return query source strings only; they do not execute queries:

| Helper | What it returns |
| ------ | --------------- |
| `get_highlights_query(language)` | `highlights.scm` source, when bundled |
| `get_injections_query(language)` | `injections.scm` source, when bundled |
| `get_locals_query(language)` | `locals.scm` source, when bundled |
| `get_tags_query(language)` | `tags.scm` source, when bundled |
| `get_indents_query(language)` | `indents.scm` source, when bundled |
| `get_folds_query(language)` | `folds.scm` source, when bundled |

If you need custom extraction, call [`get_parser()`](/guides/parsing/), parse the source with `Parser.parse(&str)` or `Parser.parse_bytes(&[u8])`, then walk the tree manually or run tree-sitter query APIs in your host language.

### Next steps

- [Code intelligence](/guides/intelligence/) — built-in extraction for common patterns
- [Parsing code](/guides/parsing/) — raw syntax trees and low-level node traversal
