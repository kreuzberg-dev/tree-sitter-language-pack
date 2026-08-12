---
id: fixture_elixir_smoke_cedar
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "cedar"}
result = TreeSitterLanguagePack.process("permit(principal, action, resource);", config_value)

```
