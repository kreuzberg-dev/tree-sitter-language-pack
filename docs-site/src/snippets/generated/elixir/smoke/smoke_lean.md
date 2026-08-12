---
id: fixture_elixir_smoke_lean
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "lean"}
result = TreeSitterLanguagePack.process("def main : IO Unit := pure ()", config_value)

```
