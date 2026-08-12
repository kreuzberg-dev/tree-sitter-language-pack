---
id: fixture_elixir_smoke_reason
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "reason"}
result = TreeSitterLanguagePack.process("let x = 1;\n", config_value)

```
