---
id: fixture_rust_detect_content_no_shebang
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::detect_language_from_content;

fn main() {
    let content = r#"no shebang here"#;
    let _ = detect_language_from_content(content);
}

```
