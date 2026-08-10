```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "ini"}
result = TreeSitterLanguagePack.process("[section]\nkey = value", config_value)

```
