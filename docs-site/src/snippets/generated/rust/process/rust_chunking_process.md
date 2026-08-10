```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"fn alpha() {}
    let config_json: serde_json::Value = serde_json::from_str(r#"{"chunk_max_size":30,"language":"rust"}"#).unwrap();
    let config = serde_json::from_value(config_json).unwrap();
    let _ = process(source, &config);
}

```
