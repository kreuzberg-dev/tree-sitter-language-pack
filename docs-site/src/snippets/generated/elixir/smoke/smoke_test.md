---
id: fixture_elixir_smoke_test
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "test"}
result = TreeSitterLanguagePack.process("===========\nTest\n===========\n---\n(node)", config_value)

```
