---
id: fixture_elixir_smoke_heex
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "heex"}
result = TreeSitterLanguagePack.process("<%= @greeting %>", config_value)

```
