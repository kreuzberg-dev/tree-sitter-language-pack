---
id: fixture_elixir_smoke_julia
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "julia"}
result = TreeSitterLanguagePack.process("function main() end", config_value)

```
