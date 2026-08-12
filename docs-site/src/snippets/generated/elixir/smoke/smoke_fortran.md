---
id: fixture_elixir_smoke_fortran
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "fortran"}
result = TreeSitterLanguagePack.process("program main\nend program main", config_value)

```
