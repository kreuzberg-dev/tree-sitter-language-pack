---
id: fixture_elixir_smoke_yuck
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "yuck"}
result = TreeSitterLanguagePack.process("(defwidget main [] (label :text \"hi\"))", config_value)

```
