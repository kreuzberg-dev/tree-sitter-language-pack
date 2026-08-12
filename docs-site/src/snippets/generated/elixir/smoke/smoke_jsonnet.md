---
id: fixture_elixir_smoke_jsonnet
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "jsonnet"}
result = TreeSitterLanguagePack.process("{ key: 'value' }", config_value)

```
