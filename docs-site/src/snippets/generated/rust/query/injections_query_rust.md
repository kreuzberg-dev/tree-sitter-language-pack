---
id: fixture_rust_injections_query_rust
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::get_injections_query;

fn main() {
    let language = r#"rust"#;
    let _ = get_injections_query(language);
}

```
