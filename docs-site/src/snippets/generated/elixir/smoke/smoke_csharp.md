---
id: fixture_elixir_smoke_csharp
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "csharp"}
result = TreeSitterLanguagePack.process("class Main {}", config_value)

```
