```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "haskell"}
result = TreeSitterLanguagePack.process("main = putStrLn \"hello\"", config_value)

```
