```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "python", symbols: true}
result = TreeSitterLanguagePack.process("MY_CONST = 42\ndef helper(): pass\nclass Widget: pass\n", config_value)

```
