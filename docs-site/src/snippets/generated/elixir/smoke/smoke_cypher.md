---
id: fixture_elixir_smoke_cypher
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "cypher"}
result = TreeSitterLanguagePack.process("MATCH (n) RETURN n\n", config_value)

```
