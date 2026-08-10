```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "gosum"}
result = TreeSitterLanguagePack.process("example.com/pkg v1.0.0 h1:abc=", config_value)

```
