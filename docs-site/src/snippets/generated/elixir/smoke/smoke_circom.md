---
id: fixture_elixir_smoke_circom
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "circom"}
result = TreeSitterLanguagePack.process("x", config_value)

```
