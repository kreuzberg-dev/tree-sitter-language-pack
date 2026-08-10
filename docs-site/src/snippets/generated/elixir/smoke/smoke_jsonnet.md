```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "jsonnet"}
result = TreeSitterLanguagePack.process("{ key: 'value' }", config_value)

```
