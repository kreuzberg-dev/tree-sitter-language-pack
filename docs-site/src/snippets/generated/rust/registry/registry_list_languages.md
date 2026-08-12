---
id: fixture_rust_registry_list_languages
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::available_languages;

fn main() {
    let _ = available_languages();
}

```
