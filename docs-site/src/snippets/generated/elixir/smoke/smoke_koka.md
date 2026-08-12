---
id: fixture_elixir_smoke_koka
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "koka"}
result = TreeSitterLanguagePack.process("fun main()\n  1\n", config_value)

```
