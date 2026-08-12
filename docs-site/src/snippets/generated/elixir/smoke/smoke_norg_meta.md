---
id: fixture_elixir_smoke_norg_meta
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "norg_meta"}
result = TreeSitterLanguagePack.process("x", config_value)

```
