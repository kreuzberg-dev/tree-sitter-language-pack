---
id: fixture_elixir_smoke_hocon
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "hocon"}
result = TreeSitterLanguagePack.process("x", config_value)

```
