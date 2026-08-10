```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "rust"}
result = TreeSitterLanguagePack.process("fn main() {}", config_value)

```
