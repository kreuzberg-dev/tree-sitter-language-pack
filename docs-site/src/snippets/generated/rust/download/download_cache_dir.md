---
id: fixture_rust_download_cache_dir
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::cache_dir;

fn main() {
    let _ = cache_dir();
}

```
