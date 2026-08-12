---
id: fixture_rust_smoke_gherkin
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use tree_sitter_language_pack::process;

fn main() {
    let source = r#"Feature: Calculator
      Scenario: Add numbers
    Given I have entered 1
    When I add 2
    Then the result should be 3
    "#;
    let config_json: serde_json::Value = serde_json::from_str(r#"{"language":"gherkin"}"#).unwrap();
    let config = serde_json::from_value(config_json).unwrap();
    let _ = process(source, &config);
}

```
