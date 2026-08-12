---
id: fixture_elixir_smoke_gleam
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "gleam"}
result = TreeSitterLanguagePack.process("pub fn main() { }", config_value)

```
