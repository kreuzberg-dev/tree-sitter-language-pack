```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "starlark"}
result = TreeSitterLanguagePack.process("def hello(): pass", config_value)

```
