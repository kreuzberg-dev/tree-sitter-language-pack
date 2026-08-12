---
id: fixture_elixir_smoke_pgn
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "pgn"}
result = TreeSitterLanguagePack.process("1. e4 e5 *", config_value)

```
