---
id: fixture_elixir_smoke_json5
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "json5"}
result = TreeSitterLanguagePack.process("x", config_value)

```
