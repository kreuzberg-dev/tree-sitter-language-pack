```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "prisma"}
result = TreeSitterLanguagePack.process("model User { id Int @id }", config_value)

```
