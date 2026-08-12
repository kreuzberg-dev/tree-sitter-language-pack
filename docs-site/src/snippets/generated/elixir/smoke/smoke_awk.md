---
id: fixture_elixir_smoke_awk
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "awk"}
result = TreeSitterLanguagePack.process("BEGIN { print \"hello\" }", config_value)

```
