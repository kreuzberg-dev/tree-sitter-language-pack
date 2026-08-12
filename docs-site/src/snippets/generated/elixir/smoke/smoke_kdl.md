---
id: fixture_elixir_smoke_kdl
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "kdl"}
result = TreeSitterLanguagePack.process("node \"value\"", config_value)

```
