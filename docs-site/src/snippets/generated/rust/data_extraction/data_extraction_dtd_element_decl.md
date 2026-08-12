---
id: fixture_rust_data_extraction_dtd_element_decl
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"<!ELEMENT server (host, port)>
    <!ELEMENT host (#PCDATA)>
    "#;
    let config_json: serde_json::Value = serde_json::from_str(r#"{"data_extraction":true,"language":"dtd"}"#).unwrap();
    let config = serde_json::from_value(config_json).unwrap();
    let _ = process(source, &config);
}

```
