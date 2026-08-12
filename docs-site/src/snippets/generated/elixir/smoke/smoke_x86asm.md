---
id: fixture_elixir_smoke_x86asm
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "x86asm"}
result = TreeSitterLanguagePack.process("x", config_value)

```
