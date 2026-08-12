---
id: fixture_elixir_smoke_wolfram
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "wolfram"}
result = TreeSitterLanguagePack.process("x = 1\n", config_value)

```
