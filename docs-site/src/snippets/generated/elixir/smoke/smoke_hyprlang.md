```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "hyprlang"}
result = TreeSitterLanguagePack.process("general { border_size = 1 }", config_value)

```
