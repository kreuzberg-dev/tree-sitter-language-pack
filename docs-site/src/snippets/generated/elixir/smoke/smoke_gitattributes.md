```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "gitattributes"}
result = TreeSitterLanguagePack.process("*.txt text", config_value)

```
