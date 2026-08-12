---
id: fixture_elixir_smoke_dockerfile
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "dockerfile"}
result = TreeSitterLanguagePack.process("FROM alpine", config_value)

```
