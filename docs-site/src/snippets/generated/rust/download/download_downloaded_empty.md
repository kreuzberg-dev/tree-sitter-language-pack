---
id: fixture_rust_download_downloaded_empty
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::downloaded_languages;

fn main() {
    let _ = downloaded_languages();
}

```
