---
id: fixture_elixir_smoke_fusion
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "fusion"}
result = TreeSitterLanguagePack.process("foo = 1\n", config_value)

```
