```rust title="Rust"
use tree_sitter_language_pack::get_highlights_query;

fn main() {
    let language = r#"rust"#;
    let _ = get_highlights_query(language);
}

```
