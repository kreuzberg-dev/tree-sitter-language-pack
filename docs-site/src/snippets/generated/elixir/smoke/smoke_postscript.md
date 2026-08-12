---
id: fixture_elixir_smoke_postscript
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "postscript"}
result = TreeSitterLanguagePack.process("/hello { (Hello) show } def", config_value)

```
