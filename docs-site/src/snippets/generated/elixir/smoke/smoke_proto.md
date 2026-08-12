---
id: fixture_elixir_smoke_proto
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "proto"}
result = TreeSitterLanguagePack.process("syntax = \"proto3\";", config_value)

```
