---
id: fixture_elixir_smoke_jq
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "jq"}
result = TreeSitterLanguagePack.process(".[] | select(.key)", config_value)

```
