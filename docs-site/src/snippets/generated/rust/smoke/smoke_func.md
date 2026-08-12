---
id: fixture_rust_smoke_func
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"() recv_internal() {}"#;
    let config_json: serde_json::Value = serde_json::from_str(r#"{"language":"func"}"#).unwrap();
    let config = serde_json::from_value(config_json).unwrap();
    let _ = process(source, &config);
}

```
