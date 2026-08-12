---
id: fixture_elixir_smoke_sas
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "sas"}
result = TreeSitterLanguagePack.process("data _null_;\nrun;\n", config_value)

```
