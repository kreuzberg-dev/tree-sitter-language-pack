---
id: fixture_rust_error_detect_extension_empty
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::detect_language_from_extension;

fn main() {
    let ext = r#""#;
    let _ = detect_language_from_extension(ext);
}

```
