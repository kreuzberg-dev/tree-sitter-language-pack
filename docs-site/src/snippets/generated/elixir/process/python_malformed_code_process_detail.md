---
id: fixture_elixir_python_malformed_code_process_detail
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{diagnostics: true, language: "python"}
result = TreeSitterLanguagePack.process("def broken(\n    return\nclass", config_value)

```
