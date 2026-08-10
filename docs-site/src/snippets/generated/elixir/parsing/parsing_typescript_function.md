```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "typescript"}
result = TreeSitterLanguagePack.process("function greet(name: string): string { return `hi ${name}`; }", config_value)

```
