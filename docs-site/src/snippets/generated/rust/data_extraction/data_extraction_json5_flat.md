---
id: fixture_rust_data_extraction_json5_flat
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"{
      host: "localhost",
      port: 8080,
}

```
