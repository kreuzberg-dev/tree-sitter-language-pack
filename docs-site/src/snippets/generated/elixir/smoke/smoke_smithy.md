```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "smithy"}
result = TreeSitterLanguagePack.process("namespace example\nstring MyString", config_value)

```
