---
id: fixture_rust_python_chunking_process_detail
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"def alpha():
    pass

    def beta():
    pass

    def gamma():
    pass

    def delta():
    pass
    "#;
    let config_json: serde_json::Value = serde_json::from_str(r#"{"chunk_max_size":30,"language":"python"}"#).unwrap();
    let config = serde_json::from_value(config_json).unwrap();
    let _ = process(source, &config);
}

```
