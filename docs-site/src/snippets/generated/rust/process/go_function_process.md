---
id: fixture_rust_go_function_process
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"package main

    import "fmt"

    func main() {
    	fmt.Println("hello")
}

```
