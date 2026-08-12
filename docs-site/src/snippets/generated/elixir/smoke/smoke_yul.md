---
id: fixture_elixir_smoke_yul
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "yul"}
result = TreeSitterLanguagePack.process("object \"C\" {\n  code {\n  }\n}\n", config_value)

```
