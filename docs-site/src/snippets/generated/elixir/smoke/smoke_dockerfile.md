```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "dockerfile"}
result = TreeSitterLanguagePack.process("FROM alpine", config_value)

```
