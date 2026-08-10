```rust title="Rust"
use tree_sitter_language_pack::get_injections_query;

fn main() {
    let language = r#"rust"#;
    let _ = get_injections_query(language);
}

```
