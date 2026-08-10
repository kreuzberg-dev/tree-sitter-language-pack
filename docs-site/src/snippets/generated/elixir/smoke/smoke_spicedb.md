```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "spicedb"}
result = TreeSitterLanguagePack.process("definition user {}\n", config_value)

```
