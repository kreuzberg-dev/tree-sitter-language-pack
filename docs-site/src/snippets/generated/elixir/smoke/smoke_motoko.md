---
id: fixture_elixir_smoke_motoko
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "motoko"}
result = TreeSitterLanguagePack.process("actor {\n}\n", config_value)

```
