---
id: fixture_elixir_smoke_gotmpl
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "gotmpl"}
result = TreeSitterLanguagePack.process("x", config_value)

```
