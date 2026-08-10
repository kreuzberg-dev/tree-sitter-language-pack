```rust title="Rust"
use tree_sitter_language_pack::get_locals_query;

fn main() {
    let language = r#"cue"#;
    let _ = get_locals_query(language);
}

```
