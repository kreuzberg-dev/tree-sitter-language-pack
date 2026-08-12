---
id: fixture_elixir_process_python_docstrings
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{docstrings: true, language: "python"}
result = TreeSitterLanguagePack.process("def greet(name):\n    \"\"\"Say hello to someone.\"\"\"\n    return f\"Hello {name}\"\n", config_value)

```
