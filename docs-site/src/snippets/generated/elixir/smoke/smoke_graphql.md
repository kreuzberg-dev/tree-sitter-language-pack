---
id: fixture_elixir_smoke_graphql
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "graphql"}
result = TreeSitterLanguagePack.process("type Query { hello: String }", config_value)

```
