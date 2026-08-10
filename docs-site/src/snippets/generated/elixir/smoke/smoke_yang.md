```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "yang"}
result = TreeSitterLanguagePack.process("module m {\n}\n", config_value)

```
