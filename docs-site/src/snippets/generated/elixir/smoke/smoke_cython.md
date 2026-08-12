---
id: fixture_elixir_smoke_cython
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "cython"}
result = TreeSitterLanguagePack.process("x = 1\n", config_value)

```
