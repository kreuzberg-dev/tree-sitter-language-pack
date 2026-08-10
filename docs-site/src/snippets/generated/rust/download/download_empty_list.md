```rust title="Rust"
use tree_sitter_language_pack::download;

fn main() {
    let names_json: serde_json::Value = serde_json::from_str(r#"[]"#).unwrap();
    let names = serde_json::from_value::<Vec<String>>(names_json).unwrap();
    let names_refs: Vec<&str> = names.iter().map(String::as_str).collect();
    let _ = download(&names_refs);
}

```
