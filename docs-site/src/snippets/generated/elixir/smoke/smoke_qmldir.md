---
id: fixture_elixir_smoke_qmldir
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "qmldir"}
result = TreeSitterLanguagePack.process("module Example", config_value)

```
