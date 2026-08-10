```rust title="Rust"
use tree_sitter_language_pack::detect_language_from_path;

fn main() {
    let path = r#"Makefile"#;
    let _ = detect_language_from_path(path);
}

```
