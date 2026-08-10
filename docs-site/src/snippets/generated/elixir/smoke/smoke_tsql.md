```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "tsql"}
result = TreeSitterLanguagePack.process("SELECT 1;\n", config_value)

```
