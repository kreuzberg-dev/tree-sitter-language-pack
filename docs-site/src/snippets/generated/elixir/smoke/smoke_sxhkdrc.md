```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "sxhkdrc"}
result = TreeSitterLanguagePack.process("super + a\n\techo hi\n", config_value)

```
