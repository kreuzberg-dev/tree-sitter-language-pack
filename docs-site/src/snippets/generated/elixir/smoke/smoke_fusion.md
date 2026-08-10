```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "fusion"}
result = TreeSitterLanguagePack.process("foo = 1\n", config_value)

```
