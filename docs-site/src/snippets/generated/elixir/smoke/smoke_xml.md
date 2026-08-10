```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "xml"}
result = TreeSitterLanguagePack.process("<?xml version=\"1.0\"?>\n<root>hello</root>", config_value)

```
