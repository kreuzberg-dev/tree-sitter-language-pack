---
id: fixture_elixir_smoke_menhir
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "menhir"}
result = TreeSitterLanguagePack.process("%token EOF\n%%\n", config_value)

```
