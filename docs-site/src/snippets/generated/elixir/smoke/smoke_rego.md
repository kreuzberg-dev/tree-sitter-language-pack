```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "rego"}
result = TreeSitterLanguagePack.process("package main\ndefault allow = false", config_value)

```
