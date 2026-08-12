---
id: fixture_rust_download_clean_cache
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::clean_cache;

fn main() {
    let _ = clean_cache();
}

```
