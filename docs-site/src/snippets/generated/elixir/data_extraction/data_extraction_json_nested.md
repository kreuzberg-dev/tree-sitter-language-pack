```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "json"}
result = TreeSitterLanguagePack.process("{\"server\": {\"host\": \"x\", \"port\": 8080}}", config_value)

```
