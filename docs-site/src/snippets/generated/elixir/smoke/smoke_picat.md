```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "picat"}
result = TreeSitterLanguagePack.process("main => true.\n", config_value)

```
