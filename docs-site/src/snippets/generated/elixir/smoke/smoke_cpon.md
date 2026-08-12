---
id: fixture_elixir_smoke_cpon
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "cpon"}
result = TreeSitterLanguagePack.process("{\"key\": 1}", config_value)

```
