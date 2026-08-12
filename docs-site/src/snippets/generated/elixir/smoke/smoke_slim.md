---
id: fixture_elixir_smoke_slim
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "slim"}
result = TreeSitterLanguagePack.process("p hello\n", config_value)

```
