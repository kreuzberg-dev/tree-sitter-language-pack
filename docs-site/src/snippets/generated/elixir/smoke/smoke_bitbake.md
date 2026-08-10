```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "bitbake"}
result = TreeSitterLanguagePack.process("DESCRIPTION = \"hello\"", config_value)

```
