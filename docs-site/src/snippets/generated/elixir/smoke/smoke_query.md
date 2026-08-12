---
id: fixture_elixir_smoke_query
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "query"}
result = TreeSitterLanguagePack.process("(identifier) @name", config_value)

```
