---
id: fixture_rust_tags_query_kotlin
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::get_tags_query;

fn main() {
    let language = r#"kotlin"#;
    let _ = get_tags_query(language);
}

```
