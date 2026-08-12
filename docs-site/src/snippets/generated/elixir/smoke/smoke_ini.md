---
id: fixture_elixir_smoke_ini
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "ini"}
result = TreeSitterLanguagePack.process("[section]\nkey = value", config_value)

```
