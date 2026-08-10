```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "xml"}
result = TreeSitterLanguagePack.process("<server id=\"main\"><host>localhost</host></server>", config_value)

```
