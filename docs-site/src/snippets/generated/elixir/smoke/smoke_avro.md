```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "avro"}
result = TreeSitterLanguagePack.process("protocol P {\n}\n", config_value)

```
