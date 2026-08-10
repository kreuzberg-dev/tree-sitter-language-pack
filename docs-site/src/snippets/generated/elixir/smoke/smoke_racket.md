```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "racket"}
result = TreeSitterLanguagePack.process("\#lang racket\n(define x 1)", config_value)

```
