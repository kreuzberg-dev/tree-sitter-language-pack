---
id: fixture_elixir_smoke_typescript
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "typescript"}
result = TreeSitterLanguagePack.process("const x: number = 42;", config_value)

```
