```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "csv"}
result = TreeSitterLanguagePack.process("a,b,c\n1,2,3", config_value)

```
