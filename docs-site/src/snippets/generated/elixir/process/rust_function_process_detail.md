```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "rust"}
result = TreeSitterLanguagePack.process("fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n", config_value)

```
