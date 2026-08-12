---
id: fixture_elixir_smoke_sparql
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "sparql"}
result = TreeSitterLanguagePack.process("SELECT ?s WHERE { ?s ?p ?o }", config_value)

```
