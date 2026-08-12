---
id: fixture_rust_smoke_ssh_config
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"Host example
      HostName example.com"#;
    let config_json: serde_json::Value = serde_json::from_str(r#"{"language":"ssh_config"}"#).unwrap();
    let config = serde_json::from_value(config_json).unwrap();
    let _ = process(source, &config);
}

```
