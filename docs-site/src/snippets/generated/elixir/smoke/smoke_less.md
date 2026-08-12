---
id: fixture_elixir_smoke_less
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "less"}
result = TreeSitterLanguagePack.process("x", config_value)

```
