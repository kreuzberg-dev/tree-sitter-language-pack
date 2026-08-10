```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "cedar"}
result = TreeSitterLanguagePack.process("permit(principal, action, resource);", config_value)

```
