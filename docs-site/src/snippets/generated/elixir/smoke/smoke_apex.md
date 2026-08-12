---
id: fixture_elixir_smoke_apex
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "apex"}
result = TreeSitterLanguagePack.process("public class Main {}", config_value)

```
