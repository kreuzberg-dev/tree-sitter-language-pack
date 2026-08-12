---
id: fixture_elixir_smoke_func
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "func"}
result = TreeSitterLanguagePack.process("() recv_internal() {}", config_value)

```
