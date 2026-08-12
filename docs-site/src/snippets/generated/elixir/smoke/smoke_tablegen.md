---
id: fixture_elixir_smoke_tablegen
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "tablegen"}
result = TreeSitterLanguagePack.process("def Hello : Base {}", config_value)

```
