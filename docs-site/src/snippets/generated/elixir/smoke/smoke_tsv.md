```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "tsv"}
result = TreeSitterLanguagePack.process("a\tb\tc\n1\t2\t3", config_value)

```
