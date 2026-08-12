---
id: fixture_elixir_smoke_css
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "css"}
result = TreeSitterLanguagePack.process("body { color: red; }", config_value)

```
