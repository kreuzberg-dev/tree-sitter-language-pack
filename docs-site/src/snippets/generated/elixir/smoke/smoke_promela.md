```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "promela"}
result = TreeSitterLanguagePack.process("init {\n}\n", config_value)

```
