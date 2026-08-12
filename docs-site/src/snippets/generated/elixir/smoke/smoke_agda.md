---
id: fixture_elixir_smoke_agda
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "agda"}
result = TreeSitterLanguagePack.process("module Main where", config_value)

```
