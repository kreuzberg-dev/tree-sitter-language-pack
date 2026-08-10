```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "haml"}
result = TreeSitterLanguagePack.process("%p hello\n", config_value)

```
