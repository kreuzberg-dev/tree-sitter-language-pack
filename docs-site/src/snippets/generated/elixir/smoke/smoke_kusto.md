```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "kusto"}
result = TreeSitterLanguagePack.process("T | count\n", config_value)

```
