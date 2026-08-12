---
id: fixture_elixir_smoke_cooklang
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "cooklang"}
result = TreeSitterLanguagePack.process("x", config_value)

```
