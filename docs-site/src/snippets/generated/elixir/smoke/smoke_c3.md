---
id: fixture_elixir_smoke_c3
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "c3"}
result = TreeSitterLanguagePack.process("x", config_value)

```
