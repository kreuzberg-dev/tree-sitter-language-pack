---
id: fixture_elixir_smoke_nim
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "nim"}
result = TreeSitterLanguagePack.process("echo \"hello\"", config_value)

```
