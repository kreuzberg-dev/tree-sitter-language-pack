---
id: fixture_rust_download_manifest_languages
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::manifest_languages;

fn main() {
    let _ = manifest_languages();
}

```
