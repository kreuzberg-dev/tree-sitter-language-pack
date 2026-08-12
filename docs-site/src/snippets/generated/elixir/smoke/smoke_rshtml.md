---
id: fixture_elixir_smoke_rshtml
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "rshtml"}
result = TreeSitterLanguagePack.process("<p>hi</p>\n", config_value)

```
