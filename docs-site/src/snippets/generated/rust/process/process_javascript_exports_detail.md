---
id: fixture_rust_process_javascript_exports_detail
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"export function greet(name) {
      return `Hello ${name}`;
}

```
