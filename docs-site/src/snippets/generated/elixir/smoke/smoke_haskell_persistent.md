```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "haskell_persistent"}
result = TreeSitterLanguagePack.process("Person\n  name String\n", config_value)

```
