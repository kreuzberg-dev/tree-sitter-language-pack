---
id: fixture_rust_detect_ext_java
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::detect_language_from_extension;

fn main() {
    let ext = r#"java"#;
    let _ = detect_language_from_extension(ext);
}

```
