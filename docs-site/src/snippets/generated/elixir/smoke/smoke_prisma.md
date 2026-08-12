---
id: fixture_elixir_smoke_prisma
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "prisma"}
result = TreeSitterLanguagePack.process("model User { id Int @id }", config_value)

```
