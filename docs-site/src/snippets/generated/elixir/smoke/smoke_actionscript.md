---
id: fixture_elixir_smoke_actionscript
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "actionscript"}
result = TreeSitterLanguagePack.process("var x:int = 1;", config_value)

```
