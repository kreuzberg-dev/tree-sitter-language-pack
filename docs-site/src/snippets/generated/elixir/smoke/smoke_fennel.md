---
id: fixture_elixir_smoke_fennel
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "fennel"}
result = TreeSitterLanguagePack.process("(fn hello [] (print :hello))", config_value)

```
