```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "motoko"}
result = TreeSitterLanguagePack.process("actor {\n}\n", config_value)

```
