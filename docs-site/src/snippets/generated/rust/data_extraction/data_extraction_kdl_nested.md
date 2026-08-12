---
id: fixture_rust_data_extraction_kdl_nested
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"server {
      host "localhost"
      port 8080
}

```
