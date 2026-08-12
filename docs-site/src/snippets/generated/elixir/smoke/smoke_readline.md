---
id: fixture_elixir_smoke_readline
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "readline"}
result = TreeSitterLanguagePack.process("set editing-mode vi", config_value)

```
