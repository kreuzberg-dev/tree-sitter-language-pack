---
id: fixture_elixir_data_extraction_hcl_attribute
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "hcl"}
result = TreeSitterLanguagePack.process("region = \"us-east-1\"\ncount  = 3\n", config_value)

```
