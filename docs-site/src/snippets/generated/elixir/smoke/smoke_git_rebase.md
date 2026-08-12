---
id: fixture_elixir_smoke_git_rebase
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "git_rebase"}
result = TreeSitterLanguagePack.process("x", config_value)

```
