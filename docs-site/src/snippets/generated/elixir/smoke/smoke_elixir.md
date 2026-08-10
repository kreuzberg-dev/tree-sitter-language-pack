```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "elixir"}
result = TreeSitterLanguagePack.process("IO.puts(\"hello\")", config_value)

```
