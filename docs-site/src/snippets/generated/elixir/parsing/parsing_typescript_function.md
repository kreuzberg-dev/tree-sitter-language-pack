---
id: fixture_elixir_parsing_typescript_function
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "typescript"}
result = TreeSitterLanguagePack.process("function greet(name: string): string { return `hi ${name}`; }", config_value)

```
