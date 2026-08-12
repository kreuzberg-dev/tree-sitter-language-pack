---
id: fixture_elixir_smoke_magik
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "magik"}
result = TreeSitterLanguagePack.process("_method object.hello\n_endmethod", config_value)

```
