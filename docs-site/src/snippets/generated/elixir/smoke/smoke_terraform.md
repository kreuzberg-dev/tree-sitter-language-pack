---
id: fixture_elixir_smoke_terraform
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "terraform"}
result = TreeSitterLanguagePack.process("resource \"null_resource\" \"main\" {}", config_value)

```
