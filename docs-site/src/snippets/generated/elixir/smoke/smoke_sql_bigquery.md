---
id: fixture_elixir_smoke_sql_bigquery
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "sql_bigquery"}
result = TreeSitterLanguagePack.process("x", config_value)

```
