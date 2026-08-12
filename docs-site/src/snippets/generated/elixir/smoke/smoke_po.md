---
id: fixture_elixir_smoke_po
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "po"}
result = TreeSitterLanguagePack.process("msgid \"hello\"\nmsgstr \"world\"", config_value)

```
