---
id: fixture_elixir_smoke_eiffel
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "eiffel"}
result = TreeSitterLanguagePack.process("class FOO\nend\n", config_value)

```
