---
id: fixture_elixir_smoke_nqc
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "nqc"}
result = TreeSitterLanguagePack.process("task main() {}", config_value)

```
