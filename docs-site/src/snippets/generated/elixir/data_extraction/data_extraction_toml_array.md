---
id: fixture_elixir_data_extraction_toml_array
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "toml"}
result = TreeSitterLanguagePack.process("ports = [8080, 8081, 8082]\n", config_value)

```
