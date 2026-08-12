---
id: fixture_elixir_data_extraction_yaml_sequence
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "yaml"}
result = TreeSitterLanguagePack.process("ports:\n  - 8080\n  - 8081\n", config_value)

```
