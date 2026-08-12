---
id: fixture_elixir_smoke_t32
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "t32"}
result = TreeSitterLanguagePack.process("PRINT 1\n", config_value)

```
