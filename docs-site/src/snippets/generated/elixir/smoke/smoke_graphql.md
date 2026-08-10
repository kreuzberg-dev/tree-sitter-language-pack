```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "graphql"}
result = TreeSitterLanguagePack.process("type Query { hello: String }", config_value)

```
