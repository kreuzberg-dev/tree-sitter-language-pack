---
id: fixture_elixir_smoke_postgres
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "postgres"}
result = TreeSitterLanguagePack.process("SELECT 1;\n", config_value)

```
