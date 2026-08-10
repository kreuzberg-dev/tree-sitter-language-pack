```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "aiken"}
result = TreeSitterLanguagePack.process("fn main() {\n  1\n}\n", config_value)

```
