```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "yuck"}
result = TreeSitterLanguagePack.process("(defwidget main [] (label :text \"hi\"))", config_value)

```
