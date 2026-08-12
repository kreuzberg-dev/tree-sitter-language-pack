---
id: fixture_elixir_smoke_aiken
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "aiken"}
result = TreeSitterLanguagePack.process("fn main() {\n  1\n}\n", config_value)

```
