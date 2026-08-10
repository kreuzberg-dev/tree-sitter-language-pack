```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "test"}
result = TreeSitterLanguagePack.process("===========\nTest\n===========\n---\n(node)", config_value)

```
