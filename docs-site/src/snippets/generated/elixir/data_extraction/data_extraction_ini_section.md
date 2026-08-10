```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "ini"}
result = TreeSitterLanguagePack.process("[database]\nhost=localhost\nport=5432\n", config_value)

```
