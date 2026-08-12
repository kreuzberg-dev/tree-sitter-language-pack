---
id: fixture_rust_python_class_with_methods_process_detail
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"class Calculator:
    def add(self, a, b):
        return a + b

    def subtract(self, a, b):
        return a - b
    "#;
    let config_json: serde_json::Value = serde_json::from_str(r#"{"language":"python"}"#).unwrap();
    let config = serde_json::from_value(config_json).unwrap();
    let _ = process(source, &config);
}

```
