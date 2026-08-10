```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "pascal"}
result = TreeSitterLanguagePack.process("program Hello; begin end.", config_value)

```
