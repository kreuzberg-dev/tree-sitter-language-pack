---
id: fixture_elixir_smoke_ispc
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "ispc"}
result = TreeSitterLanguagePack.process("export void main() {}", config_value)

```
