---
id: fixture_elixir_smoke_pascal
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "pascal"}
result = TreeSitterLanguagePack.process("program Hello; begin end.", config_value)

```
