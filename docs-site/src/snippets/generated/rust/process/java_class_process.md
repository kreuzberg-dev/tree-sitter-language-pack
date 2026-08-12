---
id: fixture_rust_java_class_process
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"import java.util.List;

    public class Greeter {
    public String greet(String name) {
        return "Hello " + name;
    }
}

```
