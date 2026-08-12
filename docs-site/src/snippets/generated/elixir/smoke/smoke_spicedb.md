---
id: fixture_elixir_smoke_spicedb
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "spicedb"}
result = TreeSitterLanguagePack.process("definition user {}\n", config_value)

```
