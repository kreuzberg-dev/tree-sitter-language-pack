---
id: fixture_elixir_smoke_requirements
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "requirements"}
result = TreeSitterLanguagePack.process("flask>=2.0", config_value)

```
