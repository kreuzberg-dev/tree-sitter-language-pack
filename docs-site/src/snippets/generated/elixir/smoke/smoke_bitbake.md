---
id: fixture_elixir_smoke_bitbake
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "bitbake"}
result = TreeSitterLanguagePack.process("DESCRIPTION = \"hello\"", config_value)

```
