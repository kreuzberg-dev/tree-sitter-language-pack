```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "typescript"}
result = TreeSitterLanguagePack.process("const x: number = 42;", config_value)

```
