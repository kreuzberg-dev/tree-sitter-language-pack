```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"import java.util.List;
    public String greet(String name) {
        return "Hello " + name;
    }
    let config_json: serde_json::Value = serde_json::from_str(r#"{"language":"java"}"#).unwrap();
    let config = serde_json::from_value(config_json).unwrap();
    let _ = process(source, &config);
}

```
