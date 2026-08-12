---
id: fixture_elixir_smoke_linkerscript
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "linkerscript"}
result = TreeSitterLanguagePack.process("SECTIONS { .text : { *(.text) } }", config_value)

```
