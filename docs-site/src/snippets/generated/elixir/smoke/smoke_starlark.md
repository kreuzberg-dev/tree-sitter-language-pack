---
id: fixture_elixir_smoke_starlark
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "starlark"}
result = TreeSitterLanguagePack.process("def hello(): pass", config_value)

```
