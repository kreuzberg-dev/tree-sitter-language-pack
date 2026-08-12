---
id: fixture_elixir_smoke_kitty
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "kitty"}
result = TreeSitterLanguagePack.process("font_size 12\n", config_value)

```
