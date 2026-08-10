```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "go"}
result = TreeSitterLanguagePack.process("package main\nfunc main() {}", config_value)

```
