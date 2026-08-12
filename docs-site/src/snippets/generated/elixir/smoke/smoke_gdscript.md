---
id: fixture_elixir_smoke_gdscript
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "gdscript"}
result = TreeSitterLanguagePack.process("extends Node\nfunc _ready():\n\tpass", config_value)

```
