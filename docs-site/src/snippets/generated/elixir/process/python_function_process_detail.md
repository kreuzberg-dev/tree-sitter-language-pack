---
id: fixture_elixir_python_function_process_detail
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "python"}
result = TreeSitterLanguagePack.process("def greet(name):\n    return f'Hello, {name}!'\n", config_value)

```
