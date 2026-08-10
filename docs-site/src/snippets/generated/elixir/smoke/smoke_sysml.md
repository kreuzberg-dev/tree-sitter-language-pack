```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "sysml"}
result = TreeSitterLanguagePack.process("package P {}\n", config_value)

```
