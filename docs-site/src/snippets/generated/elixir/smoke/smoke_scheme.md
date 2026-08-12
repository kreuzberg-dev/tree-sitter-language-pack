---
id: fixture_elixir_smoke_scheme
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "scheme"}
result = TreeSitterLanguagePack.process("(define x 1)", config_value)

```
