```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "gitignore"}
result = TreeSitterLanguagePack.process("*.o\n*.log", config_value)

```
