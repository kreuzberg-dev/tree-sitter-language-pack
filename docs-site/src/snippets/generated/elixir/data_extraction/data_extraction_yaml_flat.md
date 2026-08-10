```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "yaml"}
result = TreeSitterLanguagePack.process("host: localhost\nport: 8080\n", config_value)

```
