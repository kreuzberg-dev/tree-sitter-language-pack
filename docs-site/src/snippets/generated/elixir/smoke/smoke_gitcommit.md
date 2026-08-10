```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "gitcommit"}
result = TreeSitterLanguagePack.process("feat: add feature\n\nBody text", config_value)

```
