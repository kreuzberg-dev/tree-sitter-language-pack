```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "po"}
result = TreeSitterLanguagePack.process("msgid \"hello\"\nmsgstr \"world\"", config_value)

```
