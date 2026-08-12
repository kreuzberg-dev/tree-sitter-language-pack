---
id: fixture_elixir_smoke_xquery
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "xquery"}
result = TreeSitterLanguagePack.process("1\n", config_value)

```
