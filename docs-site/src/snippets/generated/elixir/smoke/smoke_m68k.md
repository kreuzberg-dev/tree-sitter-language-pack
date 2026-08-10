```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "m68k"}
result = TreeSitterLanguagePack.process(" move.l d0,d1\n", config_value)

```
