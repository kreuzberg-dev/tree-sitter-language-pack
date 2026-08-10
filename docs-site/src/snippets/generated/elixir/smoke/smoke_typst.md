```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "typst"}
result = TreeSitterLanguagePack.process("\#let x = 1", config_value)

```
