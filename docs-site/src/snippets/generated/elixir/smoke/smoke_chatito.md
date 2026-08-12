---
id: fixture_elixir_smoke_chatito
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "chatito"}
result = TreeSitterLanguagePack.process("%[greeting]\n    hello", config_value)

```
