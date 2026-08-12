---
id: fixture_rust_rust_chunking_process_detail
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"fn alpha() {}

    fn beta() {}

    fn gamma() {}

    fn delta() {}
    "#;
    let config_json: serde_json::Value = serde_json::from_str(r#"{"chunk_max_size":30,"language":"rust"}"#).unwrap();
    let config = serde_json::from_value(config_json).unwrap();
    let _ = process(source, &config);
}

```
