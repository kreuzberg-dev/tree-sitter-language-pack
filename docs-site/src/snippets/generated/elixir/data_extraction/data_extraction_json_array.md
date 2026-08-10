```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "json"}
result = TreeSitterLanguagePack.process("[1, 2, 3]", config_value)

```
