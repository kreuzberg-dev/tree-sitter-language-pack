---
id: fixture_elixir_smoke_printf
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "printf"}
result = TreeSitterLanguagePack.process("%d %s", config_value)

```
