```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "arduino"}
result = TreeSitterLanguagePack.process("void setup() {}", config_value)

```
