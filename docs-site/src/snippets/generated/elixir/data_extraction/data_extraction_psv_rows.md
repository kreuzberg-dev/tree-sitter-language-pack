---
id: fixture_elixir_data_extraction_psv_rows
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "psv"}
result = TreeSitterLanguagePack.process("a|b|c\n1|2|3\n", config_value)

```
