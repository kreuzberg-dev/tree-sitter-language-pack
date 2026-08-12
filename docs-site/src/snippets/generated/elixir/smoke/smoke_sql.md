---
id: fixture_elixir_smoke_sql
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "sql"}
result = TreeSitterLanguagePack.process("SELECT 1;", config_value)

```
