---
id: fixture_elixir_smoke_xit
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "xit"}
result = TreeSitterLanguagePack.process("[ ] todo\n", config_value)

```
