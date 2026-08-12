---
id: fixture_elixir_smoke_smalltalk
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "smalltalk"}
result = TreeSitterLanguagePack.process("x", config_value)

```
