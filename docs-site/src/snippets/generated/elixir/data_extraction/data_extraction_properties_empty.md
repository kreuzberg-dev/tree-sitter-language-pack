---
id: fixture_elixir_data_extraction_properties_empty
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "properties"}
result = TreeSitterLanguagePack.process("", config_value)

```
