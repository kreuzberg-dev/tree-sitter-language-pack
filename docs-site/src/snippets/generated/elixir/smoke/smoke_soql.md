```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "soql"}
result = TreeSitterLanguagePack.process("SELECT Id FROM Account\n", config_value)

```
