```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "edoc"}
result = TreeSitterLanguagePack.process("@doc foo\n", config_value)

```
