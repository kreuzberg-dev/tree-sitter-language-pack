---
id: fixture_rust_registry_get_parser_alias
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::get_parser;

fn main() {
    let name = r#"shell"#;
    let _ = get_parser(name);
}

```
