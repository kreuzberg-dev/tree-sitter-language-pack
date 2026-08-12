---
id: fixture_elixir_smoke_cfml
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "cfml"}
result = TreeSitterLanguagePack.process("x", config_value)

```
