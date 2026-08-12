---
id: fixture_elixir_smoke_bpftrace
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "bpftrace"}
result = TreeSitterLanguagePack.process("BEGIN { }\n", config_value)

```
