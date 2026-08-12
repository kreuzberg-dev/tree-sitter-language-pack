---
id: fixture_elixir_smoke_properties
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "properties"}
result = TreeSitterLanguagePack.process("key=value", config_value)

```
