---
id: fixture_elixir_smoke_janet
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "janet"}
result = TreeSitterLanguagePack.process("(print \"hello\")", config_value)

```
