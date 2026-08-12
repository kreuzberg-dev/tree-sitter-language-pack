---
id: fixture_elixir_smoke_templ
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "templ"}
result = TreeSitterLanguagePack.process("x", config_value)

```
