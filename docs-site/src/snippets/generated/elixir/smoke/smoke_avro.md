---
id: fixture_elixir_smoke_avro
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "avro"}
result = TreeSitterLanguagePack.process("protocol P {\n}\n", config_value)

```
