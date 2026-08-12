---
id: fixture_elixir_smoke_devicetree
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "devicetree"}
result = TreeSitterLanguagePack.process("/dts-v1/;\n/ { };", config_value)

```
