---
id: fixture_elixir_smoke_gitattributes
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "gitattributes"}
result = TreeSitterLanguagePack.process("*.txt text", config_value)

```
