---
id: fixture_rust_prefetch_languages
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::prefetch;

fn main() {
    let languages_json: serde_json::Value = serde_json::from_str(r#"["python"]"#).unwrap();
    let languages = serde_json::from_value::<Vec<String>>(languages_json).unwrap();
    let languages_refs: Vec<&str> = languages.iter().map(String::as_str).collect();
    let _ = prefetch(&languages_refs);
}

```
