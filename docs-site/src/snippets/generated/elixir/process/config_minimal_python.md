```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "python"}
result = TreeSitterLanguagePack.process("def hello():\n    pass\n", config_value)

```
