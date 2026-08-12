---
id: fixture_elixir_smoke_wdl
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "wdl"}
result = TreeSitterLanguagePack.process("version 1.0\n", config_value)

```
