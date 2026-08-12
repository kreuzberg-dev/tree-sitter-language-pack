---
id: fixture_elixir_smoke_elm
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "elm"}
result = TreeSitterLanguagePack.process("module Main exposing (..)", config_value)

```
