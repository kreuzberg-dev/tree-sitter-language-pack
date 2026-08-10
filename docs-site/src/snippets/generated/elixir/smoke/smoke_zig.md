```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "zig"}
result = TreeSitterLanguagePack.process("pub fn main() void {}", config_value)

```
