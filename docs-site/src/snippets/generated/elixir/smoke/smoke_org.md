```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "org"}
result = TreeSitterLanguagePack.process("* Hello\nWorld", config_value)

```
