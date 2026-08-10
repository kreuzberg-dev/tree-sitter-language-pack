```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "hcl"}
result = TreeSitterLanguagePack.process("variable \"name\" { type = string }", config_value)

```
