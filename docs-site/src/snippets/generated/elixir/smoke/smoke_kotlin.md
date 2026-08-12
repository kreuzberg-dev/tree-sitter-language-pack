---
id: fixture_elixir_smoke_kotlin
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "kotlin"}
result = TreeSitterLanguagePack.process("fun main() {}", config_value)

```
