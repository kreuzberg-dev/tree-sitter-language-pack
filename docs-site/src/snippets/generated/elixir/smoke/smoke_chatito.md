```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "chatito"}
result = TreeSitterLanguagePack.process("%[greeting]\n    hello", config_value)

```
