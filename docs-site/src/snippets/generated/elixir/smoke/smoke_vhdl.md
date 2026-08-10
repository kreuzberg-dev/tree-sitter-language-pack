```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "vhdl"}
result = TreeSitterLanguagePack.process("entity main is end main;", config_value)

```
