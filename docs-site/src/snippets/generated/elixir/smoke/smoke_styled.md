```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "styled"}
result = TreeSitterLanguagePack.process("color: red;\n", config_value)

```
