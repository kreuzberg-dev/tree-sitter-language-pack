```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "vb"}
result = TreeSitterLanguagePack.process("Module Main\nEnd Module", config_value)

```
