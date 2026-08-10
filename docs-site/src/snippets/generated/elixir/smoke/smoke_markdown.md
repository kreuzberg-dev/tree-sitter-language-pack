```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "markdown"}
result = TreeSitterLanguagePack.process("\# Hello\n\nWorld", config_value)

```
