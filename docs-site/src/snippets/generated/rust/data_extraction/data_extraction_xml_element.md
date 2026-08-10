```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"<server id="main"><host>localhost</host></server>"#;
    let config_json: serde_json::Value = serde_json::from_str(r#"{"data_extraction":true,"language":"xml"}"#).unwrap();
    let config = serde_json::from_value(config_json).unwrap();
    let _ = process(source, &config);
}

```
