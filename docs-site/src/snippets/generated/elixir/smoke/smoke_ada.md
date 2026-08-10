```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "ada"}
result = TreeSitterLanguagePack.process("procedure Main is begin null; end Main;", config_value)

```
