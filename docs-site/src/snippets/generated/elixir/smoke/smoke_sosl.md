---
id: fixture_elixir_smoke_sosl
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "sosl"}
result = TreeSitterLanguagePack.process("FIND {test}\n", config_value)

```
