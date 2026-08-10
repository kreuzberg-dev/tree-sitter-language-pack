```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "sosl"}
result = TreeSitterLanguagePack.process("FIND {test}\n", config_value)

```
