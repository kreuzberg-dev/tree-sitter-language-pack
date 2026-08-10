```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"void main() { gl_Position = vec4(0.0); }"#;
    let config_json: serde_json::Value = serde_json::from_str(r#"{"language":"glsl"}"#).unwrap();
    let config = serde_json::from_value(config_json).unwrap();
    let _ = process(source, &config);
}

```
