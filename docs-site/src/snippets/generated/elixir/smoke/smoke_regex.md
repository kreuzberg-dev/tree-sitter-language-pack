---
id: fixture_elixir_smoke_regex
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "regex"}
result = TreeSitterLanguagePack.process("x", config_value)

```
