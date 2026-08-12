---
id: fixture_elixir_smoke_promela
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "promela"}
result = TreeSitterLanguagePack.process("init {\n}\n", config_value)

```
