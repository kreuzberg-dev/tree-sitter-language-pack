---
id: fixture_rust_rust_function_process
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"fn add(a: i32, b: i32) -> i32 {
    a + b
}

```
