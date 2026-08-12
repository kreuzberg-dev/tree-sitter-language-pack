---
id: fixture_elixir_smoke_godot_resource
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "godot_resource"}
result = TreeSitterLanguagePack.process("x", config_value)

```
