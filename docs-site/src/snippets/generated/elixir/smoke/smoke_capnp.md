---
id: fixture_elixir_smoke_capnp
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "capnp"}
result = TreeSitterLanguagePack.process("@0xabcdef1234567890;", config_value)

```
