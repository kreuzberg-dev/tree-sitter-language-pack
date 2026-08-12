---
id: fixture_elixir_smoke_svelte
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "svelte"}
result = TreeSitterLanguagePack.process("<script>let x = 1;</script>", config_value)

```
