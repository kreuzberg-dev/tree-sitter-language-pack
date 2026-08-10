```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "po"}
result = TreeSitterLanguagePack.process("msgid \"Hello\"\nmsgstr \"Hallo\"\n", config_value)

```
