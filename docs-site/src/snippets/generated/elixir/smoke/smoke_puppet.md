```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "puppet"}
result = TreeSitterLanguagePack.process("notify { 'hello': }", config_value)

```
