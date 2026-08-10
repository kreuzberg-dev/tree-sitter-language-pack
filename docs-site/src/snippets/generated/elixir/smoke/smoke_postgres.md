```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "postgres"}
result = TreeSitterLanguagePack.process("SELECT 1;\n", config_value)

```
