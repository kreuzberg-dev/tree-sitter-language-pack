```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "jinja2"}
result = TreeSitterLanguagePack.process("{{ variable }}", config_value)

```
