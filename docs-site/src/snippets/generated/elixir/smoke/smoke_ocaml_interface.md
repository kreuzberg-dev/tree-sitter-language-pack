---
id: fixture_elixir_smoke_ocaml_interface
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "ocaml_interface"}
result = TreeSitterLanguagePack.process("val x : int", config_value)

```
