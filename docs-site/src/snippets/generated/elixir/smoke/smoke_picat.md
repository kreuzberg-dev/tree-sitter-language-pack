---
id: fixture_elixir_smoke_picat
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "picat"}
result = TreeSitterLanguagePack.process("main => true.\n", config_value)

```
