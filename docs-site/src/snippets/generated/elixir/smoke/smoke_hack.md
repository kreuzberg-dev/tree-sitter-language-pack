---
id: fixture_elixir_smoke_hack
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "hack"}
result = TreeSitterLanguagePack.process("<?hh\nfunction main(): void {}", config_value)

```
