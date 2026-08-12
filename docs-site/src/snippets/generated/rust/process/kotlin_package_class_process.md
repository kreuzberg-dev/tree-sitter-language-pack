---
id: fixture_rust_kotlin_package_class_process
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"package foo.bar

    class Widget {
    fun greet(): String = "hi"
}

```
