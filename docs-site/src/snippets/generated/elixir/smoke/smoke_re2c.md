---
id: fixture_elixir_smoke_re2c
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "re2c"}
result = TreeSitterLanguagePack.process("/*!re2c\n  [a-z]+ { return; }\n*/", config_value)

```
