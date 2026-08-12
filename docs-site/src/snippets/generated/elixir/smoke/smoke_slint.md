---
id: fixture_elixir_smoke_slint
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "slint"}
result = TreeSitterLanguagePack.process("export component Foo {}\n", config_value)

```
