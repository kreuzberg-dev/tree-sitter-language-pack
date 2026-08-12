---
id: fixture_elixir_smoke_c
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "c"}
result = TreeSitterLanguagePack.process("int main() { return 0; }", config_value)

```
