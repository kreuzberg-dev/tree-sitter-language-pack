---
id: fixture_elixir_parsing_go_function
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "go"}
result = TreeSitterLanguagePack.process("package main\nfunc main() {}", config_value)

```
