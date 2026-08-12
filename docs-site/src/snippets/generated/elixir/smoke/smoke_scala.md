---
id: fixture_elixir_smoke_scala
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "scala"}
result = TreeSitterLanguagePack.process("object Main", config_value)

```
