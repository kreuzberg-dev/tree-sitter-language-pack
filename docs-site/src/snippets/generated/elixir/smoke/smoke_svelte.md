```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "svelte"}
result = TreeSitterLanguagePack.process("<script>let x = 1;</script>", config_value)

```
