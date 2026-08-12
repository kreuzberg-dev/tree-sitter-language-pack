---
id: fixture_elixir_smoke_editorconfig
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "editorconfig"}
result = TreeSitterLanguagePack.process("x", config_value)

```
