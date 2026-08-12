---
id: fixture_elixir_smoke_wat
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "wat"}
result = TreeSitterLanguagePack.process("(module)", config_value)

```
