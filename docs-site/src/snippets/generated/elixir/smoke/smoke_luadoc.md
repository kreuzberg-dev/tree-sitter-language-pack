---
id: fixture_elixir_smoke_luadoc
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "luadoc"}
result = TreeSitterLanguagePack.process("---@param name string", config_value)

```
