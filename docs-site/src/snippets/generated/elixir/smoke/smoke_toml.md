```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "toml"}
result = TreeSitterLanguagePack.process("key = \"value\"", config_value)

```
