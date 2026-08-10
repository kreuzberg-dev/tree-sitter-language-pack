```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "sql"}
result = TreeSitterLanguagePack.process("SELECT 1;", config_value)

```
