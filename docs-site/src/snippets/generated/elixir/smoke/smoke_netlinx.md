---
id: fixture_elixir_smoke_netlinx
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "netlinx"}
result = TreeSitterLanguagePack.process("PROGRAM_NAME='hello'", config_value)

```
