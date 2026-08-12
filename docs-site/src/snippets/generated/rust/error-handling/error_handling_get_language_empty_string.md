---
id: fixture_rust_error_handling_get_language_empty_string
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::get_language;

fn main() {
    let name = r#""#;
    let _ = get_language(name);
}

```
