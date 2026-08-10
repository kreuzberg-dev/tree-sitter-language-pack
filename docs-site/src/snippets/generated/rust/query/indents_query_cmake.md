```rust title="Rust"
use tree_sitter_language_pack::get_indents_query;

fn main() {
    let language = r#"cmake"#;
    let _ = get_indents_query(language);
}

```
