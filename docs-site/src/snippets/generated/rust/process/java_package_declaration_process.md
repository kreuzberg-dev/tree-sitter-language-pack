---
id: fixture_rust_java_package_declaration_process
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"package com.example.widget;

    public class Widget {
    public String name() { return "w"; }
}

```
