---
id: fixture_elixir_smoke_asm
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "asm"}
result = TreeSitterLanguagePack.process("mov eax, 1", config_value)

```
