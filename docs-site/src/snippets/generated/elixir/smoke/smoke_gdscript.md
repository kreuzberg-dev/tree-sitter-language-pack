```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "gdscript"}
result = TreeSitterLanguagePack.process("extends Node\nfunc _ready():\n\tpass", config_value)

```
