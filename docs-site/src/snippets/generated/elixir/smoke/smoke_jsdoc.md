---
id: fixture_elixir_smoke_jsdoc
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "jsdoc"}
result = TreeSitterLanguagePack.process("/** @param {string} name */", config_value)

```
