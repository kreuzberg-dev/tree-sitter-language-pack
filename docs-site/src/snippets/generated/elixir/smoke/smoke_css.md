```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "css"}
result = TreeSitterLanguagePack.process("body { color: red; }", config_value)

```
