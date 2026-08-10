```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "slint"}
result = TreeSitterLanguagePack.process("export component Foo {}\n", config_value)

```
