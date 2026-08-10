```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "magik"}
result = TreeSitterLanguagePack.process("_method object.hello\n_endmethod", config_value)

```
