---
id: fixture_elixir_smoke_rst
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "rst"}
result = TreeSitterLanguagePack.process("Hello\n=====\n\nWorld", config_value)

```
