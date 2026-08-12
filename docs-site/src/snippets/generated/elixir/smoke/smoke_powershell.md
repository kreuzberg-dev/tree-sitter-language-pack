---
id: fixture_elixir_smoke_powershell
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "powershell"}
result = TreeSitterLanguagePack.process("Write-Host 'hello'", config_value)

```
