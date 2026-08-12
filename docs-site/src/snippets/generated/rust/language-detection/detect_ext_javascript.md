---
id: fixture_rust_detect_ext_javascript
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::detect_language_from_extension;

fn main() {
    let ext = r#"js"#;
    let _ = detect_language_from_extension(ext);
}

```
