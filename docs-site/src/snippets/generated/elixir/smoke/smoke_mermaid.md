```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "mermaid"}
result = TreeSitterLanguagePack.process("graph TD\nA --> B", config_value)

```
