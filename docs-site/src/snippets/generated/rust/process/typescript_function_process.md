---
id: fixture_rust_typescript_function_process
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"import { readFile } from 'fs';

    function greet(name: string): string {
    return `Hello, ${name}!`;
}

```
