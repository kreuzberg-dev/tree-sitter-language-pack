---
id: fixture_elixir_smoke_scss
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "scss"}
result = TreeSitterLanguagePack.process("$color: red;\nbody { color: $color; }", config_value)

```
