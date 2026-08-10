```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "clarity"}
result = TreeSitterLanguagePack.process("(define-public (hello) (ok true))", config_value)

```
