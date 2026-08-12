---
id: fixture_elixir_smoke_puppet
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "puppet"}
result = TreeSitterLanguagePack.process("notify { 'hello': }", config_value)

```
