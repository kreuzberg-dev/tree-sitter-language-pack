---
id: fixture_elixir_smoke_hyprlang
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "hyprlang"}
result = TreeSitterLanguagePack.process("general { border_size = 1 }", config_value)

```
