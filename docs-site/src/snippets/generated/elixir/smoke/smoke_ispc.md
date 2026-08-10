```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "ispc"}
result = TreeSitterLanguagePack.process("export void main() {}", config_value)

```
