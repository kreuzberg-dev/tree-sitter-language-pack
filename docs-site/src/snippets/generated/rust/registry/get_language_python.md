---
id: fixture_rust_get_language_python
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::get_language;

fn main() {
    let name = r#"python"#;
    let _ = get_language(name);
}

```
