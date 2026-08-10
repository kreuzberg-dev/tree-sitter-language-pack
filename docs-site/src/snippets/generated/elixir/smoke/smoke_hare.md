```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "hare"}
result = TreeSitterLanguagePack.process("export fn main() void = void;", config_value)

```
