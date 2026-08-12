---
id: fixture_elixir_smoke_luap
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "luap"}
result = TreeSitterLanguagePack.process("[a-z]+", config_value)

```
