---
id: fixture_elixir_smoke_tsv
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "tsv"}
result = TreeSitterLanguagePack.process("a\tb\tc\n1\t2\t3", config_value)

```
