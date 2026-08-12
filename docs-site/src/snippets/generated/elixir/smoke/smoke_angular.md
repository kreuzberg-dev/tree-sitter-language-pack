---
id: fixture_elixir_smoke_angular
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "angular"}
result = TreeSitterLanguagePack.process("x", config_value)

```
