---
id: fixture_elixir_smoke_uxntal
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "uxntal"}
result = TreeSitterLanguagePack.process("|0100 LIT 01", config_value)

```
