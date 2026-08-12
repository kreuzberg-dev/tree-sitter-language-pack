---
id: fixture_elixir_smoke_gren
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "gren"}
result = TreeSitterLanguagePack.process("module Main exposing (..)", config_value)

```
