```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "javascript"}
result = TreeSitterLanguagePack.process("function function function @@@ %%%", config_value)

```
