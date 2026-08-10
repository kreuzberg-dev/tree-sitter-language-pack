```rust title="Rust"
use tree_sitter_language_pack::get_parser;

fn main() {
    let name = r#"nonexistent_xyz"#;
    let _ = get_parser(name);
}

```
