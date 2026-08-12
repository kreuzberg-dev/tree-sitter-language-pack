---
id: fixture_elixir_smoke_jjdescription
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "jjdescription"}
result = TreeSitterLanguagePack.process("commit message\n", config_value)

```
