```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "reason"}
result = TreeSitterLanguagePack.process("let x = 1;\n", config_value)

```
