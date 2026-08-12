---
id: fixture_elixir_smoke_task
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "task"}
result = TreeSitterLanguagePack.process("todo item\n", config_value)

```
