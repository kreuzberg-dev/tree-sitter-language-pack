---
id: fixture_elixir_smoke_mermaid
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "mermaid"}
result = TreeSitterLanguagePack.process("graph TD\nA --> B", config_value)

```
