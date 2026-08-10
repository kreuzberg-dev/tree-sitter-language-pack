```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "applescript"}
result = TreeSitterLanguagePack.process("set x to 1\n", config_value)

```
