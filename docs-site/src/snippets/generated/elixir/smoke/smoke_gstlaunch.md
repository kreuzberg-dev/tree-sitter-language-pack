---
id: fixture_elixir_smoke_gstlaunch
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "gstlaunch"}
result = TreeSitterLanguagePack.process("fakesrc ! fakesink", config_value)

```
