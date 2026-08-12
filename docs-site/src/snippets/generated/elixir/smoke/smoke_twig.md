---
id: fixture_elixir_smoke_twig
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "twig"}
result = TreeSitterLanguagePack.process("{{ variable }}", config_value)

```
