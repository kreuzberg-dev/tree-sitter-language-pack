---
id: fixture_elixir_data_extraction_csv_single_row
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "csv"}
result = TreeSitterLanguagePack.process("x,y,z\n", config_value)

```
