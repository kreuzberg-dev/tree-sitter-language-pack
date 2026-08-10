```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "kconfig"}
result = TreeSitterLanguagePack.process("config FOO\n\tbool \"Enable foo\"", config_value)

```
