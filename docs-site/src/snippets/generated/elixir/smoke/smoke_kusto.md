---
id: fixture_elixir_smoke_kusto
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "kusto"}
result = TreeSitterLanguagePack.process("T | count\n", config_value)

```
