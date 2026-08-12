---
id: fixture_elixir_smoke_koto
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "koto"}
result = TreeSitterLanguagePack.process("x = 1\n", config_value)

```
