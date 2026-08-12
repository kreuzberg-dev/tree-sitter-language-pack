---
id: fixture_elixir_smoke_ballerina
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "ballerina"}
result = TreeSitterLanguagePack.process("public function main() {\n}\n", config_value)

```
