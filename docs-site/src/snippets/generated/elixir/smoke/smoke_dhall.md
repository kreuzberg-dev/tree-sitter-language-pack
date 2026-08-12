---
id: fixture_elixir_smoke_dhall
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "dhall"}
result = TreeSitterLanguagePack.process("x", config_value)

```
