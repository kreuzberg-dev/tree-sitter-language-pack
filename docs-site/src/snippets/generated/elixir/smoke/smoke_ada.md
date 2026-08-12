---
id: fixture_elixir_smoke_ada
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "ada"}
result = TreeSitterLanguagePack.process("procedure Main is begin null; end Main;", config_value)

```
