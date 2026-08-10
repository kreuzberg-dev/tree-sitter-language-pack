```rust title="Rust"
use tree_sitter_language_pack::get_highlights_query;

fn main() {
    let language = r#"zzz_nonexistent_lang"#;
    let _ = get_highlights_query(language);
}

```
