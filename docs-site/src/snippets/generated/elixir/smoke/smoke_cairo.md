```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "cairo"}
result = TreeSitterLanguagePack.process("fn main() {}", config_value)

```
