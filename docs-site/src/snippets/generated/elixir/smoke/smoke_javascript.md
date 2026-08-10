```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "javascript"}
result = TreeSitterLanguagePack.process("console.log('hello');", config_value)

```
