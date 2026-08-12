---
id: fixture_elixir_smoke_astro
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "astro"}
result = TreeSitterLanguagePack.process("---\n---\n<p>hello</p>", config_value)

```
