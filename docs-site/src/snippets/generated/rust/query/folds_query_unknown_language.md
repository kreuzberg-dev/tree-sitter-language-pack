---
id: fixture_rust_folds_query_unknown_language
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::get_folds_query;

fn main() {
    let language = r#"nonexistent_xyz"#;
    let _ = get_folds_query(language);
}

```
