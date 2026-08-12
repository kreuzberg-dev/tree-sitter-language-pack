---
id: fixture_elixir_smoke_tlaplus
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "tlaplus"}
result = TreeSitterLanguagePack.process("---- MODULE Main ----\n====", config_value)

```
