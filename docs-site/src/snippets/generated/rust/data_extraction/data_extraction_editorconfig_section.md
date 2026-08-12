---
id: fixture_rust_data_extraction_editorconfig_section
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"[*.rs]
    indent_style = space
    indent_size = 4
    "#;
    let config_json: serde_json::Value = serde_json::from_str(r#"{"data_extraction":true,"language":"editorconfig"}"#).unwrap();
    let config = serde_json::from_value(config_json).unwrap();
    let _ = process(source, &config);
}

```
