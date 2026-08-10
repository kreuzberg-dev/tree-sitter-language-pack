```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "powershell"}
result = TreeSitterLanguagePack.process("Write-Host 'hello'", config_value)

```
