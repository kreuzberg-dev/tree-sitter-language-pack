---
id: fixture_elixir_smoke_gcode
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "gcode"}
result = TreeSitterLanguagePack.process("G0 X0\n", config_value)

```
