---
id: fixture_elixir_smoke_sysml
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "sysml"}
result = TreeSitterLanguagePack.process("package P {}\n", config_value)

```
