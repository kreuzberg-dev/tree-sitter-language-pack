---
id: fixture_elixir_smoke_lalrpop
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "lalrpop"}
result = TreeSitterLanguagePack.process("grammar;\n", config_value)

```
