---
id: fixture_elixir_smoke_fish
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "fish"}
result = TreeSitterLanguagePack.process("echo hello", config_value)

```
