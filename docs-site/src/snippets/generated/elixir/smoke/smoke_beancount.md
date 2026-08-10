```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "beancount"}
result = TreeSitterLanguagePack.process("2024-01-01 open Assets:Bank USD", config_value)

```
