---
id: fixture_elixir_smoke_ocaml
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "ocaml"}
result = TreeSitterLanguagePack.process("let () = print_endline \"hello\"", config_value)

```
