---
id: fixture_elixir_smoke_pkl
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "pkl"}
result = TreeSitterLanguagePack.process("name = \"hello\"", config_value)

```
