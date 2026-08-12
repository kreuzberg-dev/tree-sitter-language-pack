---
id: fixture_elixir_smoke_markdown_inline
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "markdown_inline"}
result = TreeSitterLanguagePack.process("**bold** and *italic*", config_value)

```
