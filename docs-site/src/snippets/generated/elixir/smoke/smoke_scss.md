```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "scss"}
result = TreeSitterLanguagePack.process("$color: red;\nbody { color: $color; }", config_value)

```
