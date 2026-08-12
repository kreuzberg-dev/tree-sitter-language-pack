---
id: fixture_elixir_error_handling_invalid_syntax
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "javascript"}
result = TreeSitterLanguagePack.process("function function function @@@ %%%", config_value)

```
