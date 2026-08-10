```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "xresources"}
result = TreeSitterLanguagePack.process("*.foreground: \#ffffff\n", config_value)

```
