```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "moonbit"}
result = TreeSitterLanguagePack.process("fn main {\n}\n", config_value)

```
