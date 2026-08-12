---
id: fixture_elixir_smoke_tera
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "tera"}
result = TreeSitterLanguagePack.process("x", config_value)

```
