```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "csv"}
result = TreeSitterLanguagePack.process("x,y,z\n", config_value)

```
