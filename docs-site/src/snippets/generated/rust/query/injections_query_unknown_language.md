```rust title="Rust"
use tree_sitter_language_pack::get_injections_query;

fn main() {
    let language = r#"nonexistent_xyz"#;
    let _ = get_injections_query(language);
}

```
