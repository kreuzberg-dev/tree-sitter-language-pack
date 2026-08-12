---
id: fixture_elixir_smoke_ledger
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "ledger"}
result = TreeSitterLanguagePack.process("x", config_value)

```
