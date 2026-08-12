---
id: fixture_elixir_smoke_markdown
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "markdown"}
result = TreeSitterLanguagePack.process("\# Hello\n\nWorld", config_value)

```
