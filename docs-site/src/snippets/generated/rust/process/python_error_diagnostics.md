---
id: fixture_rust_python_error_diagnostics
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"def broken(
    pass
    "#;
    let config_json: serde_json::Value = serde_json::from_str(r#"{"diagnostics":true,"language":"python"}"#).unwrap();
    let config = serde_json::from_value(config_json).unwrap();
    let _ = process(source, &config);
}

```
