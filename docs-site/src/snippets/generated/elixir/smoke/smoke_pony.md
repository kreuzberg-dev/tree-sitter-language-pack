---
id: fixture_elixir_smoke_pony
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "pony"}
result = TreeSitterLanguagePack.process("actor Main\n  new create(env: Env) => None", config_value)

```
