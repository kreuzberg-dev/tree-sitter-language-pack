```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "fluent"}
result = TreeSitterLanguagePack.process("hello = Hello\n", config_value)

```
