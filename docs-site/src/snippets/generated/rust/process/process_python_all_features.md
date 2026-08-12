---
id: fixture_rust_process_python_all_features
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"import os
    from pathlib import Path

    # Configuration
    MY_CONST = 42

    def process_file(path):
    """Process a file and return contents."""
    with open(path) as f:
        return f.read()

    class FileProcessor:
    def __init__(self, base_dir):
        self.base_dir = base_dir
    "#;
    let config_json: serde_json::Value = serde_json::from_str(r#"{"comments":true,"docstrings":true,"imports":true,"language":"python","structure":true,"symbols":true}"#).unwrap();
    let config = serde_json::from_value(config_json).unwrap();
    let _ = process(source, &config);
}

```
