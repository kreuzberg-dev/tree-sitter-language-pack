---
id: fixture_elixir_smoke_hoon
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "hoon"}
result = TreeSitterLanguagePack.process("x", config_value)

```
