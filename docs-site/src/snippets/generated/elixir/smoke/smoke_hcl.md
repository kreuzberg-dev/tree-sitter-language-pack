---
id: fixture_elixir_smoke_hcl
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "hcl"}
result = TreeSitterLanguagePack.process("variable \"name\" { type = string }", config_value)

```
