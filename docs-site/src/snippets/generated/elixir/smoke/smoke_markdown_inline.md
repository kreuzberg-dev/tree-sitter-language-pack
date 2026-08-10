```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "markdown_inline"}
result = TreeSitterLanguagePack.process("**bold** and *italic*", config_value)

```
