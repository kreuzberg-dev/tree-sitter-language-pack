```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "tablegen"}
result = TreeSitterLanguagePack.process("def Hello : Base {}", config_value)

```
