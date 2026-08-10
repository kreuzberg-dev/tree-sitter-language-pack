```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "dotenv"}
result = TreeSitterLanguagePack.process("KEY=value\n", config_value)

```
