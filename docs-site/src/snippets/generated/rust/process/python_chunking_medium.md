---
id: fixture_rust_python_chunking_medium
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"def first():
    x = 1
    return x

    def second():
    y = 2
    return y

    def third():
    z = 3
    return z
    "#;
    let config_json: serde_json::Value = serde_json::from_str(r#"{"chunk_max_size":50,"language":"python"}"#).unwrap();
    let config = serde_json::from_value(config_json).unwrap();
    let _ = process(source, &config);
}

```
