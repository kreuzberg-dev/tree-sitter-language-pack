---
id: fixture_elixir_smoke_vala
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "vala"}
result = TreeSitterLanguagePack.process("class Foo {\n}\n", config_value)

```
