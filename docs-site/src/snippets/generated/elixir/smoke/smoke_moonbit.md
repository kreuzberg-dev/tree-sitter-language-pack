---
id: fixture_elixir_smoke_moonbit
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "moonbit"}
result = TreeSitterLanguagePack.process("fn main {\n}\n", config_value)

```
