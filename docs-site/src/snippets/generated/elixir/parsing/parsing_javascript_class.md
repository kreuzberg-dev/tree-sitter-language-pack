```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "javascript"}
result = TreeSitterLanguagePack.process("class Foo { bar() {} }", config_value)

```
