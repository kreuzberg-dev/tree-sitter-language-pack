```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "asm"}
result = TreeSitterLanguagePack.process("mov eax, 1", config_value)

```
