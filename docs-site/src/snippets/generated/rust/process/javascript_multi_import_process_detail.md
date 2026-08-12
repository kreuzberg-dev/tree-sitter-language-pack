---
id: fixture_rust_javascript_multi_import_process_detail
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"import fs from 'fs';
    import path from 'path';

    function process(input) {
    return input.trim();
}

```
