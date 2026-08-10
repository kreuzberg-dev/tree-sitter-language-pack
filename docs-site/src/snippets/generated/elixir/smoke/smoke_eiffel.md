```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "eiffel"}
result = TreeSitterLanguagePack.process("class FOO\nend\n", config_value)

```
