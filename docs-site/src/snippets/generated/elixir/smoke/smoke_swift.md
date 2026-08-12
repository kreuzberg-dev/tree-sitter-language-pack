---
id: fixture_elixir_smoke_swift
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "swift"}
result = TreeSitterLanguagePack.process("print(\"hello\")", config_value)

```
