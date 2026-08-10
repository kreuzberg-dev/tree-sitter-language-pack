```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"digraph G { A -> B; }"#;
    let config_json: serde_json::Value = serde_json::from_str(r#"{"language":"dot"}"#).unwrap();
    let config = serde_json::from_value(config_json).unwrap();
    let _ = process(source, &config);
}

```
