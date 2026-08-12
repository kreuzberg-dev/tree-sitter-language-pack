---
id: fixture_elixir_smoke_caddy
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "caddy"}
result = TreeSitterLanguagePack.process(":8080 {\n\trespond \"Hello\"\n}", config_value)

```
