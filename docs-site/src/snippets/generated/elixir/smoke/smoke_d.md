---
id: fixture_elixir_smoke_d
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "d"}
result = TreeSitterLanguagePack.process("void main() {}", config_value)

```
