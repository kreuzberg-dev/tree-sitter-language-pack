```rust title="Rust"
use tree_sitter_language_pack::detect_language_from_content;

fn main() {
    let content = r#"no shebang here"#;
    let _ = detect_language_from_content(content);
}

```
