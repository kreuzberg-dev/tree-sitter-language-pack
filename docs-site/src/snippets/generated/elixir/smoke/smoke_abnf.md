```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "abnf"}
result = TreeSitterLanguagePack.process("a = \"b\"\r\n", config_value)

```
