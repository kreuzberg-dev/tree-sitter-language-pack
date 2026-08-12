---
id: fixture_elixir_smoke_jai
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "jai"}
result = TreeSitterLanguagePack.process("x", config_value)

```
