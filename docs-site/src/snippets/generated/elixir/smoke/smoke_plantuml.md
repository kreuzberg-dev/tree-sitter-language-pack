---
id: fixture_elixir_smoke_plantuml
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "plantuml"}
result = TreeSitterLanguagePack.process("@startuml\n@enduml\n", config_value)

```
