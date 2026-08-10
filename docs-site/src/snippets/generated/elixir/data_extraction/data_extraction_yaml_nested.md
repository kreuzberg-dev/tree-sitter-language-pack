```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "yaml"}
result = TreeSitterLanguagePack.process("server:\n  host: localhost\n  port: 8080\n", config_value)

```
