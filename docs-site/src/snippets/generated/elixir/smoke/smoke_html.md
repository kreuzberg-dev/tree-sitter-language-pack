---
id: fixture_elixir_smoke_html
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "html"}
result = TreeSitterLanguagePack.process("<p>hello</p>", config_value)

```
