```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "luadoc"}
result = TreeSitterLanguagePack.process("---@param name string", config_value)

```
