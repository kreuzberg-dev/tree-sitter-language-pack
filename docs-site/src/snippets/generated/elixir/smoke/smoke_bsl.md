---
id: fixture_elixir_smoke_bsl
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "bsl"}
result = TreeSitterLanguagePack.process("Procedure Main() EndProcedure", config_value)

```
