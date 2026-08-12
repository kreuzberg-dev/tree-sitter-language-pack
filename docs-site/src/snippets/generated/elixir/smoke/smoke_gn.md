---
id: fixture_elixir_smoke_gn
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "gn"}
result = TreeSitterLanguagePack.process("group(\"hello\") {}", config_value)

```
