---
id: fixture_elixir_smoke_r
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "r"}
result = TreeSitterLanguagePack.process("print('hello')", config_value)

```
