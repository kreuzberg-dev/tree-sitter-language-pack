---
id: fixture_elixir_smoke_haskell_persistent
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "haskell_persistent"}
result = TreeSitterLanguagePack.process("Person\n  name String\n", config_value)

```
