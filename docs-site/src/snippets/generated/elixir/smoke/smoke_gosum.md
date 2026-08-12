---
id: fixture_elixir_smoke_gosum
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "gosum"}
result = TreeSitterLanguagePack.process("example.com/pkg v1.0.0 h1:abc=", config_value)

```
