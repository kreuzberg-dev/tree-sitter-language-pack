---
id: fixture_elixir_smoke_leo
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "leo"}
result = TreeSitterLanguagePack.process("program test.aleo {\n}\n", config_value)

```
