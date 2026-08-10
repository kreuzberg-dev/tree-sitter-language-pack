```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "tlaplus"}
result = TreeSitterLanguagePack.process("---- MODULE Main ----\n====", config_value)

```
