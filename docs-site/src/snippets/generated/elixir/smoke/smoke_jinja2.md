---
id: fixture_elixir_smoke_jinja2
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "jinja2"}
result = TreeSitterLanguagePack.process("{{ variable }}", config_value)

```
