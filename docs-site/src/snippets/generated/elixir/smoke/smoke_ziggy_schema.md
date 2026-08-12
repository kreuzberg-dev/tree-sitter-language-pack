---
id: fixture_elixir_smoke_ziggy_schema
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "ziggy_schema"}
result = TreeSitterLanguagePack.process("x", config_value)

```
