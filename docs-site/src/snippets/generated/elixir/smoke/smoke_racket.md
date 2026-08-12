---
id: fixture_elixir_smoke_racket
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "racket"}
result = TreeSitterLanguagePack.process("\#lang racket\n(define x 1)", config_value)

```
