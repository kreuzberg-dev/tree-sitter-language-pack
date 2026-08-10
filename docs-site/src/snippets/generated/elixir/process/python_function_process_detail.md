```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "python"}
result = TreeSitterLanguagePack.process("def greet(name):\n    return f'Hello, {name}!'\n", config_value)

```
