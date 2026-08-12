---
id: fixture_elixir_smoke_penrose
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "penrose"}
result = TreeSitterLanguagePack.process("type Set\n", config_value)

```
