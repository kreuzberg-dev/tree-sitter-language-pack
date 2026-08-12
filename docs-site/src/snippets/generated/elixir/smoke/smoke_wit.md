---
id: fixture_elixir_smoke_wit
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "wit"}
result = TreeSitterLanguagePack.process("package example:pkg;", config_value)

```
