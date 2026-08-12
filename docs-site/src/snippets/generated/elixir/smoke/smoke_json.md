---
id: fixture_elixir_smoke_json
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "json"}
result = TreeSitterLanguagePack.process("{\"key\": \"value\"}", config_value)

```
