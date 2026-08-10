```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "javascript"}
result = TreeSitterLanguagePack.process("const x = 1;", config_value)

```
