---
id: fixture_elixir_smoke_systemtap
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "systemtap"}
result = TreeSitterLanguagePack.process("probe begin {}\n", config_value)

```
