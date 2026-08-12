---
id: fixture_elixir_smoke_abnf
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "abnf"}
result = TreeSitterLanguagePack.process("a = \"b\"\r\n", config_value)

```
