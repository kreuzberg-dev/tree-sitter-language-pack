---
id: fixture_elixir_smoke_vento
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "vento"}
result = TreeSitterLanguagePack.process("hello\n", config_value)

```
