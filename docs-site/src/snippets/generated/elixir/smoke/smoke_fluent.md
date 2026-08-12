---
id: fixture_elixir_smoke_fluent
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "fluent"}
result = TreeSitterLanguagePack.process("hello = Hello\n", config_value)

```
