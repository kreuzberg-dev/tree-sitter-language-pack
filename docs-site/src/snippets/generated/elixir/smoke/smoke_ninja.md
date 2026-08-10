```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "ninja"}
result = TreeSitterLanguagePack.process("rule cc\n  command = cc $in -o $out", config_value)

```
