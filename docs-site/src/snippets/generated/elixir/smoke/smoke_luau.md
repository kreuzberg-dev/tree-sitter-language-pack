---
id: fixture_elixir_smoke_luau
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "luau"}
result = TreeSitterLanguagePack.process("local x: number = 1", config_value)

```
