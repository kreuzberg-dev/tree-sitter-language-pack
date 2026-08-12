---
id: fixture_elixir_smoke_comment
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "comment"}
result = TreeSitterLanguagePack.process("Review: handle edge case", config_value)

```
