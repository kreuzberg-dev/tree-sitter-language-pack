```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "julia"}
result = TreeSitterLanguagePack.process("function main() end", config_value)

```
