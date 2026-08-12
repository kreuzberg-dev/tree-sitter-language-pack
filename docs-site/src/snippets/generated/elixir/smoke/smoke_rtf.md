---
id: fixture_elixir_smoke_rtf
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "rtf"}
result = TreeSitterLanguagePack.process("{\\rtf1 hello}", config_value)

```
