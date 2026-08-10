```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{diagnostics: true, language: "python"}
result = TreeSitterLanguagePack.process("def broken(\n    pass\n", config_value)

```
