```rust title="Rust"
use tree_sitter_language_pack::has_language;

fn main() {
    let name = r#"nonexistent"#;
    let _ = has_language(name);
}

```
