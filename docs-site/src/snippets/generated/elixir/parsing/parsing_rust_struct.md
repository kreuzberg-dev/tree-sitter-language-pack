---
id: fixture_elixir_parsing_rust_struct
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "rust"}
result = TreeSitterLanguagePack.process("struct Point { x: f64, y: f64 }", config_value)

```
