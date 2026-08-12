---
id: fixture_elixir_smoke_go
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "go"}
result = TreeSitterLanguagePack.process("package main", config_value)

```
