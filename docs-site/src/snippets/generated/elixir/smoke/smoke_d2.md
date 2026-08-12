---
id: fixture_elixir_smoke_d2
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "d2"}
result = TreeSitterLanguagePack.process("a -> b\n", config_value)

```
