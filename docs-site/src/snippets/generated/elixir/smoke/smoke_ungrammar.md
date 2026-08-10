```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "ungrammar"}
result = TreeSitterLanguagePack.process("Root = Item*\nItem = 'token'", config_value)

```
