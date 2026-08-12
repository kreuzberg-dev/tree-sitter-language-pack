---
id: fixture_elixir_smoke_gitcommit
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "gitcommit"}
result = TreeSitterLanguagePack.process("feat: add feature\n\nBody text", config_value)

```
