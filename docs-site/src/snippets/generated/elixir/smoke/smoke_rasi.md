---
id: fixture_elixir_smoke_rasi
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "rasi"}
result = TreeSitterLanguagePack.process("x", config_value)

```
