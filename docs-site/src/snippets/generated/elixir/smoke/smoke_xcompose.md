---
id: fixture_elixir_smoke_xcompose
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "xcompose"}
result = TreeSitterLanguagePack.process("<Multi_key> <a> : \"a\"", config_value)

```
