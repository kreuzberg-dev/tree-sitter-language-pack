```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "editorconfig"}
result = TreeSitterLanguagePack.process("[*.rs]\nindent_style = space\nindent_size = 4\n", config_value)

```
