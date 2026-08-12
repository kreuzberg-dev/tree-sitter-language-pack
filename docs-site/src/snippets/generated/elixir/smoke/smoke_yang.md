---
id: fixture_elixir_smoke_yang
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "yang"}
result = TreeSitterLanguagePack.process("module m {\n}\n", config_value)

```
