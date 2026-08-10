```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "gleam"}
result = TreeSitterLanguagePack.process("pub fn main() { }", config_value)

```
