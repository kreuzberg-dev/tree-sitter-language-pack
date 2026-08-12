---
id: fixture_elixir_smoke_dot
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "dot"}
result = TreeSitterLanguagePack.process("digraph G { A -> B; }", config_value)

```
