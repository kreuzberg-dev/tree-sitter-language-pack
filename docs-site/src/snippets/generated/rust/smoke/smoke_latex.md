---
id: fixture_rust_smoke_latex
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"\documentclass{article}
    \begin{document}
    Hello
    \end{document}"#;
    let config_json: serde_json::Value = serde_json::from_str(r#"{"language":"latex"}"#).unwrap();
    let config = serde_json::from_value(config_json).unwrap();
    let _ = process(source, &config);
}

```
