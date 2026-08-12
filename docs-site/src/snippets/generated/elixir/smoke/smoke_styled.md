---
id: fixture_elixir_smoke_styled
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "styled"}
result = TreeSitterLanguagePack.process("color: red;\n", config_value)

```
