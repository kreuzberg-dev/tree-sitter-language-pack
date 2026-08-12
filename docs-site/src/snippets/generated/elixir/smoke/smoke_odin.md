---
id: fixture_elixir_smoke_odin
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "odin"}
result = TreeSitterLanguagePack.process("package main", config_value)

```
