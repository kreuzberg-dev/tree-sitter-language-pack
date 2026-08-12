---
id: fixture_elixir_smoke_desktop
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "desktop"}
result = TreeSitterLanguagePack.process("x", config_value)

```
