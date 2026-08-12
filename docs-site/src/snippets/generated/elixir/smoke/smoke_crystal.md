---
id: fixture_elixir_smoke_crystal
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "crystal"}
result = TreeSitterLanguagePack.process("x", config_value)

```
