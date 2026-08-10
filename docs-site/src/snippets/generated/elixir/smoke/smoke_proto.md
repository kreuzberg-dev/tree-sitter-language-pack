```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "proto"}
result = TreeSitterLanguagePack.process("syntax = \"proto3\";", config_value)

```
