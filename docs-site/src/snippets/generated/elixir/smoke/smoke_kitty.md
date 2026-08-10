```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "kitty"}
result = TreeSitterLanguagePack.process("font_size 12\n", config_value)

```
