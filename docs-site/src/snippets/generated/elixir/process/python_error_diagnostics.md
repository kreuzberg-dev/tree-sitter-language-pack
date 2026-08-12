---
id: fixture_elixir_python_error_diagnostics
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{diagnostics: true, language: "python"}
result = TreeSitterLanguagePack.process("def broken(\n    pass\n", config_value)

```
