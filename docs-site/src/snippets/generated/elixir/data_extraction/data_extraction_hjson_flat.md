```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "hjson"}
result = TreeSitterLanguagePack.process("{\n  host: \"localhost\"\n  port: 8080\n}\n", config_value)

```
