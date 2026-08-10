```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "erlang"}
result = TreeSitterLanguagePack.process("main() -> ok.", config_value)

```
