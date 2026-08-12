---
id: fixture_elixir_smoke_doxygen
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "doxygen"}
result = TreeSitterLanguagePack.process("/** @brief A function */", config_value)

```
