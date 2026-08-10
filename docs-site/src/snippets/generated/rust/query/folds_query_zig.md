```rust title="Rust"
use tree_sitter_language_pack::get_folds_query;

fn main() {
    let language = r#"zig"#;
    let _ = get_folds_query(language);
}

```
