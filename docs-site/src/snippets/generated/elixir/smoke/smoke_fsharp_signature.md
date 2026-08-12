---
id: fixture_elixir_smoke_fsharp_signature
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "fsharp_signature"}
result = TreeSitterLanguagePack.process("val x: int", config_value)

```
