---
id: fixture_elixir_smoke_psv
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "psv"}
result = TreeSitterLanguagePack.process("a|b|c\n1|2|3", config_value)

```
