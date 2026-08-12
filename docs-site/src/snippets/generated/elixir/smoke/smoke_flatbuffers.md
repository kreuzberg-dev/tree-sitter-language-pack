---
id: fixture_elixir_smoke_flatbuffers
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "flatbuffers"}
result = TreeSitterLanguagePack.process("table Foo {}\n", config_value)

```
