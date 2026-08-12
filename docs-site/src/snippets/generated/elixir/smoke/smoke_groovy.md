---
id: fixture_elixir_smoke_groovy
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "groovy"}
result = TreeSitterLanguagePack.process("def x = 1", config_value)

```
