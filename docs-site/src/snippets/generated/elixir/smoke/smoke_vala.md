```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "vala"}
result = TreeSitterLanguagePack.process("class Foo {\n}\n", config_value)

```
