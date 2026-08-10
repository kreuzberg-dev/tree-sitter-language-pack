```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "janet"}
result = TreeSitterLanguagePack.process("(print \"hello\")", config_value)

```
