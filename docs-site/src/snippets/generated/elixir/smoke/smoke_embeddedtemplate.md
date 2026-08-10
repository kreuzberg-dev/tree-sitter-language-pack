```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "embeddedtemplate"}
result = TreeSitterLanguagePack.process("<%= value %>", config_value)

```
