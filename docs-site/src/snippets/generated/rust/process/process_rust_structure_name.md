```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"pub struct MyConfig {
    pub name: String,
    pub value: i32,
    pub fn new() -> Self {
        Self { name: String::new(), value: 0 }
    }
    let config_json: serde_json::Value = serde_json::from_str(r#"{"language":"rust"}"#).unwrap();
    let config = serde_json::from_value(config_json).unwrap();
    let _ = process(source, &config);
}

```
