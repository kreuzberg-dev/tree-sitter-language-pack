---
id: fixture_elixir_smoke_rust
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "rust"}
result = TreeSitterLanguagePack.process("fn main() {}", config_value)

```
