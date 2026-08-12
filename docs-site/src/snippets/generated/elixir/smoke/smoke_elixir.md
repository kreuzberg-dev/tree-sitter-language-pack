---
id: fixture_elixir_smoke_elixir
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "elixir"}
result = TreeSitterLanguagePack.process("IO.puts(\"hello\")", config_value)

```
