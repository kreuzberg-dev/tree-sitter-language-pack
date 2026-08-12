---
id: fixture_elixir_smoke_smithy
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "smithy"}
result = TreeSitterLanguagePack.process("namespace example\nstring MyString", config_value)

```
