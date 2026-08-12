---
id: fixture_elixir_smoke_org
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "org"}
result = TreeSitterLanguagePack.process("* Hello\nWorld", config_value)

```
