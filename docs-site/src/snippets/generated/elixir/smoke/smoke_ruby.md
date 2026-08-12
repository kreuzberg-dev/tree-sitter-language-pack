---
id: fixture_elixir_smoke_ruby
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "ruby"}
result = TreeSitterLanguagePack.process("puts 'hello'", config_value)

```
