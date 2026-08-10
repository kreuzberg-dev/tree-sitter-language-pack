```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "jq"}
result = TreeSitterLanguagePack.process(".[] | select(.key)", config_value)

```
