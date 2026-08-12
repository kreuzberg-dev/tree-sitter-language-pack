---
id: fixture_elixir_smoke_typst
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "typst"}
result = TreeSitterLanguagePack.process("\#let x = 1", config_value)

```
