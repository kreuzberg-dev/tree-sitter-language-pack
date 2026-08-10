```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "luau"}
result = TreeSitterLanguagePack.process("local x: number = 1", config_value)

```
