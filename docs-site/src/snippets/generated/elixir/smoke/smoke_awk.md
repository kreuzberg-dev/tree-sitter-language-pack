```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "awk"}
result = TreeSitterLanguagePack.process("BEGIN { print \"hello\" }", config_value)

```
