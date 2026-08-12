---
id: fixture_elixir_smoke_embeddedtemplate
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "embeddedtemplate"}
result = TreeSitterLanguagePack.process("<%= value %>", config_value)

```
