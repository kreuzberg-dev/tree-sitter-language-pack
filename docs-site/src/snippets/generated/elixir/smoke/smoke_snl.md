---
id: fixture_elixir_smoke_snl
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "snl"}
result = TreeSitterLanguagePack.process("program test\n", config_value)

```
