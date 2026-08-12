---
id: fixture_elixir_smoke_cst
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "cst"}
result = TreeSitterLanguagePack.process("x", config_value)

```
