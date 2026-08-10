```rust title="Rust"
use tree_sitter_language_pack::init;

fn main() {
    let config_json: serde_json::Value = serde_json::from_str(r#"{}"#).unwrap();
    let config = serde_json::from_value(config_json).unwrap();
    let _ = init(&config);
}

```
