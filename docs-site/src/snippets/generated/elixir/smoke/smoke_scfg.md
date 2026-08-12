---
id: fixture_elixir_smoke_scfg
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "scfg"}
result = TreeSitterLanguagePack.process("key value\n", config_value)

```
