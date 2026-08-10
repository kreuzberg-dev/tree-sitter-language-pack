```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "kdl"}
result = TreeSitterLanguagePack.process("server {\n  host \"localhost\"\n  port 8080\n}\n", config_value)

```
