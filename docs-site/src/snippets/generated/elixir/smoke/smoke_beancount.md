---
id: fixture_elixir_smoke_beancount
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "beancount"}
result = TreeSitterLanguagePack.process("2024-01-01 open Assets:Bank USD", config_value)

```
