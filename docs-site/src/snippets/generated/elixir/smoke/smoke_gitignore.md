---
id: fixture_elixir_smoke_gitignore
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "gitignore"}
result = TreeSitterLanguagePack.process("*.o\n*.log", config_value)

```
