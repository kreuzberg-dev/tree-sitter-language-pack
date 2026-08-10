```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "rst"}
result = TreeSitterLanguagePack.process("Hello\n=====\n\nWorld", config_value)

```
