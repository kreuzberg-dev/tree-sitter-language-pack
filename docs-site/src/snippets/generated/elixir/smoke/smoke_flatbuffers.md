```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "flatbuffers"}
result = TreeSitterLanguagePack.process("table Foo {}\n", config_value)

```
