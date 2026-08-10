```rust title="Rust"
use tree_sitter_language_pack::detect_language_from_extension;

fn main() {
    let ext = r#"java"#;
    let _ = detect_language_from_extension(ext);
}

```
