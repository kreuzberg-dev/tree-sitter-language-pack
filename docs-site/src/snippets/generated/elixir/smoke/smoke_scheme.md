```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "scheme"}
result = TreeSitterLanguagePack.process("(define x 1)", config_value)

```
