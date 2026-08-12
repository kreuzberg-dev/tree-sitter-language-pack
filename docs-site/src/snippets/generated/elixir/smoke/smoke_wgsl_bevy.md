---
id: fixture_elixir_smoke_wgsl_bevy
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "wgsl_bevy"}
result = TreeSitterLanguagePack.process("x", config_value)

```
