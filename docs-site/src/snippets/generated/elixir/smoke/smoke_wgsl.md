```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "wgsl"}
result = TreeSitterLanguagePack.process("@vertex fn main() -> @builtin(position) vec4f { return vec4f(); }", config_value)

```
