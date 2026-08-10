```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "yaml"}
result = TreeSitterLanguagePack.process("ports:\n  - 8080\n  - 8081\n", config_value)

```
