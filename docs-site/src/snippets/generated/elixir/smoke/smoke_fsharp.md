---
id: fixture_elixir_smoke_fsharp
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "fsharp"}
result = TreeSitterLanguagePack.process("let x = 1", config_value)

```
