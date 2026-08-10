```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "udev"}
result = TreeSitterLanguagePack.process("ACTION==\"add\", KERNEL==\"sd*\"", config_value)

```
