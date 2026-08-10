```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "heex"}
result = TreeSitterLanguagePack.process("<%= @greeting %>", config_value)

```
