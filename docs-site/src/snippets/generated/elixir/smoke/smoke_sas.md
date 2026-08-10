```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "sas"}
result = TreeSitterLanguagePack.process("data _null_;\nrun;\n", config_value)

```
