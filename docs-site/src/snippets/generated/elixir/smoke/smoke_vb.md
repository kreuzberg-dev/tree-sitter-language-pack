---
id: fixture_elixir_smoke_vb
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "vb"}
result = TreeSitterLanguagePack.process("Module Main\nEnd Module", config_value)

```
