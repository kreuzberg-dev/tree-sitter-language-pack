---
id: fixture_elixir_smoke_make
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "make"}
result = TreeSitterLanguagePack.process("all:\n\techo hello", config_value)

```
