```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "gcode"}
result = TreeSitterLanguagePack.process("G0 X0\n", config_value)

```
