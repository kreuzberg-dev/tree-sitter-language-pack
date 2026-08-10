```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "make"}
result = TreeSitterLanguagePack.process("all:\n\techo hello", config_value)

```
