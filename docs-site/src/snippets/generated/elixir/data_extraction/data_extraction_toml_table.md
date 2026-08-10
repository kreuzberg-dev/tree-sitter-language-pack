```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "toml"}
result = TreeSitterLanguagePack.process("[server]\nhost = \"localhost\"\nport = 8080\n", config_value)

```
