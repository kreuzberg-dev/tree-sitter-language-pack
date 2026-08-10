```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "gn"}
result = TreeSitterLanguagePack.process("group(\"hello\") {}", config_value)

```
