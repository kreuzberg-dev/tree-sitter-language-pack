---
id: fixture_elixir_smoke_unison
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "unison"}
result = TreeSitterLanguagePack.process("x = 1\n", config_value)

```
