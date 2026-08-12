---
id: fixture_elixir_smoke_java
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "java"}
result = TreeSitterLanguagePack.process("class Main {}", config_value)

```
