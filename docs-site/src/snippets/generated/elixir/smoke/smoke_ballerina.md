```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "ballerina"}
result = TreeSitterLanguagePack.process("public function main() {\n}\n", config_value)

```
