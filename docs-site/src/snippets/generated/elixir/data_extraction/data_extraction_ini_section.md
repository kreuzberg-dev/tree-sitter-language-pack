---
id: fixture_elixir_data_extraction_ini_section
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "ini"}
result = TreeSitterLanguagePack.process("[database]\nhost=localhost\nport=5432\n", config_value)

```
