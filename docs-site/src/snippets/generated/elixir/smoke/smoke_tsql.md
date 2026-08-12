---
id: fixture_elixir_smoke_tsql
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "tsql"}
result = TreeSitterLanguagePack.process("SELECT 1;\n", config_value)

```
