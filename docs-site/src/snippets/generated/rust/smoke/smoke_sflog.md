---
id: fixture_rust_smoke_sflog
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"37.0 APEX_CODE,DEBUG
    16:06:58.18 (1)|EXECUTION_STARTED
    "#;
    let config_json: serde_json::Value = serde_json::from_str(r#"{"language":"sflog"}"#).unwrap();
    let config = serde_json::from_value(config_json).unwrap();
    let _ = process(source, &config);
}

```
