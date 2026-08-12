---
id: fixture_rust_get_language_unknown
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::get_language;

fn main() {
    let name = r#"nonexistent_xyz"#;
    let _ = get_language(name);
}

```
