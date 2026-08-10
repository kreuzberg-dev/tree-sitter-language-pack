```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "wdl"}
result = TreeSitterLanguagePack.process("version 1.0\n", config_value)

```
