---
id: fixture_elixir_smoke_dtd
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "dtd"}
result = TreeSitterLanguagePack.process("<!ELEMENT note (body)>", config_value)

```
