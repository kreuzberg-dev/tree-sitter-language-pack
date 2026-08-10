```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "menhir"}
result = TreeSitterLanguagePack.process("%token EOF\n%%\n", config_value)

```
