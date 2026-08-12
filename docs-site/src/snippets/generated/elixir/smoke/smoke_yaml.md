---
id: fixture_elixir_smoke_yaml
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "yaml"}
result = TreeSitterLanguagePack.process("key: value", config_value)

```
