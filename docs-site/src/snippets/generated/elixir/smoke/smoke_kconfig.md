---
id: fixture_elixir_smoke_kconfig
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "kconfig"}
result = TreeSitterLanguagePack.process("config FOO\n\tbool \"Enable foo\"", config_value)

```
