---
id: fixture_elixir_smoke_hare
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "hare"}
result = TreeSitterLanguagePack.process("export fn main() void = void;", config_value)

```
