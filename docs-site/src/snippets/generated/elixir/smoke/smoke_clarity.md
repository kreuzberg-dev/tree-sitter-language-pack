---
id: fixture_elixir_smoke_clarity
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "clarity"}
result = TreeSitterLanguagePack.process("(define-public (hello) (ok true))", config_value)

```
