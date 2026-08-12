---
id: fixture_elixir_smoke_pymanifest
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "pymanifest"}
result = TreeSitterLanguagePack.process("include *.txt", config_value)

```
