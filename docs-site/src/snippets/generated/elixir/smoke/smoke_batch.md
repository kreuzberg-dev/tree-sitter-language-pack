```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "batch"}
result = TreeSitterLanguagePack.process("@echo off\necho hello", config_value)

```
