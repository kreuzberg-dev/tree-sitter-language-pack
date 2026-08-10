```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "xml"}
result = TreeSitterLanguagePack.process("<config><host>localhost</host><port>8080</port></config>", config_value)

```
