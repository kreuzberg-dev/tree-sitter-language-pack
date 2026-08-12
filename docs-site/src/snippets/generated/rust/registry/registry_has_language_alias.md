---
id: fixture_rust_registry_has_language_alias
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::has_language;

fn main() {
    let name = r#"shell"#;
    let _ = has_language(name);
}

```
