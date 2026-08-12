---
id: fixture_rust_process_python_metrics_detail
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"# module docstring
    import os

    def hello():
    # greeting
    print('hello')

    def world():
    print('world')
    "#;
    let config_json: serde_json::Value = serde_json::from_str(r#"{"language":"python"}"#).unwrap();
    let config = serde_json::from_value(config_json).unwrap();
    let _ = process(source, &config);
}

```
