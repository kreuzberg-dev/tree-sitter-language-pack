---
id: fixture_elixir_smoke_rego
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "rego"}
result = TreeSitterLanguagePack.process("package main\ndefault allow = false", config_value)

```
