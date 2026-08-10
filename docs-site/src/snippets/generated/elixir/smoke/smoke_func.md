```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "func"}
result = TreeSitterLanguagePack.process("() recv_internal() {}", config_value)

```
