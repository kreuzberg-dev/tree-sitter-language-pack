---
id: fixture_elixir_smoke_ungrammar
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "ungrammar"}
result = TreeSitterLanguagePack.process("Root = Item*\nItem = 'token'", config_value)

```
