---
id: fixture_elixir_smoke_idl
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "idl"}
result = TreeSitterLanguagePack.process("module M {\n};\n", config_value)

```
