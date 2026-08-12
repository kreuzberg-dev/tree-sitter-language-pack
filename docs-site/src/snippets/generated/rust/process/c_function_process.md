---
id: fixture_rust_c_function_process
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"#include <stdio.h>

    int main() {
    printf("hello");
    return 0;
}

```
