---
id: fixture_elixir_smoke_sxhkdrc
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "sxhkdrc"}
result = TreeSitterLanguagePack.process("super + a\n\techo hi\n", config_value)

```
