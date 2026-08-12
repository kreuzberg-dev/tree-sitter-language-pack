---
id: fixture_elixir_smoke_v
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "v"}
result = TreeSitterLanguagePack.process("fn main() {}", config_value)

```
