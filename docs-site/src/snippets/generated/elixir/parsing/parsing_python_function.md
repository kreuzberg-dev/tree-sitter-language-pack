```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "python"}
result = TreeSitterLanguagePack.process("def hello(): pass", config_value)

```
