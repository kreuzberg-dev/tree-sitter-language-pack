---
id: fixture_elixir_smoke_arduino
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "arduino"}
result = TreeSitterLanguagePack.process("void setup() {}", config_value)

```
