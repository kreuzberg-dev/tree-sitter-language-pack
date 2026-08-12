---
id: fixture_elixir_smoke_soql
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "soql"}
result = TreeSitterLanguagePack.process("SELECT Id FROM Account\n", config_value)

```
