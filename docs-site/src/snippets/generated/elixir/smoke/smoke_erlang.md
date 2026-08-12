---
id: fixture_elixir_smoke_erlang
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "erlang"}
result = TreeSitterLanguagePack.process("main() -> ok.", config_value)

```
