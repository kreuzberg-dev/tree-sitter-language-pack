---
id: fixture_elixir_smoke_bibtex
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "bibtex"}
result = TreeSitterLanguagePack.process("@article{key, title={A}}", config_value)

```
