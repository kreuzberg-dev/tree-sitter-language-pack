```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "c"}
result = TreeSitterLanguagePack.process("int main() { return 0; }", config_value)

```
