```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"const x = 1;"#;
    let config_json: serde_json::Value = serde_json::from_str(r#"{"language":"javascript"}"#).unwrap();
    let config = serde_json::from_value(config_json).unwrap();
    let _ = process(source, &config);
}

```
