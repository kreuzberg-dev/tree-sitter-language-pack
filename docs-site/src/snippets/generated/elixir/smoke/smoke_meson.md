```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "meson"}
result = TreeSitterLanguagePack.process("project('hello', 'c')", config_value)

```
