```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "nonexistent_xyz"}
result = TreeSitterLanguagePack.process("x = 1", config_value)

```
