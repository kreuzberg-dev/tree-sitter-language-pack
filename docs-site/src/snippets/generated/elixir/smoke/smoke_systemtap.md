```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "systemtap"}
result = TreeSitterLanguagePack.process("probe begin {}\n", config_value)

```
