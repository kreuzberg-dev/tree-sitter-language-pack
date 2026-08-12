---
id: fixture_elixir_smoke_meson
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "meson"}
result = TreeSitterLanguagePack.process("project('hello', 'c')", config_value)

```
