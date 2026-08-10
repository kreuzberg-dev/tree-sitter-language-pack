```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "gomod"}
result = TreeSitterLanguagePack.process("module example.com/hello\n\ngo 1.21", config_value)

```
