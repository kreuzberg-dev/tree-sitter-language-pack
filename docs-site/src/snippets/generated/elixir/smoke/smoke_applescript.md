---
id: fixture_elixir_smoke_applescript
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "applescript"}
result = TreeSitterLanguagePack.process("set x to 1\n", config_value)

```
