```rust title="Rust"
use tree_sitter_language_pack::get_tags_query;

fn main() {
    let language = r#"rust"#;
    let _ = get_tags_query(language);
}

```
