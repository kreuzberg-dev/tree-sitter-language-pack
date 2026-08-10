```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "terraform"}
result = TreeSitterLanguagePack.process("resource \"null_resource\" \"main\" {}", config_value)

```
