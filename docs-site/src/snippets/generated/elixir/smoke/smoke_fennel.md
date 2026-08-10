```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "fennel"}
result = TreeSitterLanguagePack.process("(fn hello [] (print :hello))", config_value)

```
