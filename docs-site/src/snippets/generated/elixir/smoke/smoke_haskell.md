---
id: fixture_elixir_smoke_haskell
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "haskell"}
result = TreeSitterLanguagePack.process("main = putStrLn \"hello\"", config_value)

```
