---
id: fixture_elixir_smoke_m68k
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "m68k"}
result = TreeSitterLanguagePack.process(" move.l d0,d1\n", config_value)

```
