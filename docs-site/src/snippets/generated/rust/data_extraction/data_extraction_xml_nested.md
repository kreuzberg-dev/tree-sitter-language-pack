---
id: fixture_rust_data_extraction_xml_nested
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"<config><host>localhost</host><port>8080</port></config>"#;
    let config_json: serde_json::Value = serde_json::from_str(r#"{"data_extraction":true,"language":"xml"}"#).unwrap();
    let config = serde_json::from_value(config_json).unwrap();
    let _ = process(source, &config);
}

```
