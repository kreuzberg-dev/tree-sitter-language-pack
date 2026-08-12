---
id: fixture_elixir_smoke_vhdl
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "vhdl"}
result = TreeSitterLanguagePack.process("entity main is end main;", config_value)

```
