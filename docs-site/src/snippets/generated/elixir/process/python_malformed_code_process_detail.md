```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{diagnostics: true, language: "python"}
result = TreeSitterLanguagePack.process("def broken(\n    return\nclass", config_value)

```
