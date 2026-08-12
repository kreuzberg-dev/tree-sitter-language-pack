---
id: fixture_elixir_data_extraction_toml_table
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "toml"}
result = TreeSitterLanguagePack.process("[server]\nhost = \"localhost\"\nport = 8080\n", config_value)

```
