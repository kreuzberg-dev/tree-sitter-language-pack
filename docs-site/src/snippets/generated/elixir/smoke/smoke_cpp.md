```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "cpp"}
result = TreeSitterLanguagePack.process("int main() { return 0; }", config_value)

```
