---
id: fixture_elixir_smoke_udev
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "udev"}
result = TreeSitterLanguagePack.process("ACTION==\"add\", KERNEL==\"sd*\"", config_value)

```
